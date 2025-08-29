import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import Cookies from 'js-cookie';
import { User, LoginCredentials, AuthContextType } from '../types/auth';

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const checkAuthStatus = async () => {
      let token = localStorage.getItem('love_daily_token');
      
      if (!token) {
        const cookieToken = Cookies.get('love_daily_token');
        if (cookieToken) {
          token = cookieToken;
          localStorage.setItem('love_daily_token', token);
        }
      }

      if (token) {
        try {
          const response = await fetch('/api/daily_messages', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({ token })
          });

          if (response.ok) {
            const data = await response.json();
            const parsedData = JSON.parse(data);
            if (parsedData.Main === "ACCES GRANTED") {
              setUser({ id: '1', username: 'user' });
            } else {
              localStorage.removeItem('love_daily_token');
              Cookies.remove('love_daily_token');
            }
          } else {
            localStorage.removeItem('love_daily_token');
            Cookies.remove('love_daily_token');
          }
        } catch (error) {
          console.error('Token validation failed:', error);
          localStorage.removeItem('love_daily_token');
          Cookies.remove('love_daily_token');
        }
      }
      setLoading(false);
    };

    checkAuthStatus();
  }, []);

  const login = async (credentials: LoginCredentials): Promise<boolean> => {
    try {
      const response = await fetch('/api/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name: credentials.username,
          password: credentials.password
        })
      });

      if (!response.ok) {
        return false;
      }

      const data = await response.json();
      
      if (data.message === "Login successful" && data.token) {
        localStorage.setItem('love_daily_token', data.token);
        
        const expirationTime = new Date();
        expirationTime.setTime(expirationTime.getTime() + (23 * 60 * 60 * 1000) + (58 * 60 * 1000));
        
        Cookies.set('love_daily_token', data.token, { 
          expires: expirationTime,
          secure: false,
          sameSite: 'lax'
        });
        
        setUser({ id: data.user_id.toString(), username: credentials.username });
        return true;
      }
      return false;
    } catch (error) {
      console.error('Login error:', error);
      return false;
    }
  };

  const logout = () => {
    localStorage.removeItem('love_daily_token');
    localStorage.removeItem('love_daily_username');
    Cookies.remove('love_daily_token');
    setUser(null);
  };

  const value: AuthContextType = {
    user,
    login,
    logout,
    isAuthenticated: !!user,
    loading
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};
