import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { useAuth } from '../contexts/AuthContext';

const Login: React.FC = () => {
  const [credentials, setCredentials] = useState({ username: '', password: '' });
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');
  const { login } = useAuth();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError('');

    const success = await login(credentials);
    if (!success) {
      setError('Hibás adatok! 🥺');
    }
    setIsLoading(false);
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setCredentials({
      ...credentials,
      [e.target.name]: e.target.value
    });
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-hello-kitty-light-pink via-hello-kitty-pink to-hello-kitty-soft-pink">
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        {[...Array(15)].map((_, i) => (
          <motion.div
            key={i}
            className="absolute text-hello-kitty-pink opacity-20"
            style={{
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
              fontSize: `${Math.random() * 12 + 8}px`,
            }}
            animate={{
              y: [0, -15, 0],
              rotate: [0, 180, 360],
              scale: [0.8, 1, 0.8],
            }}
            transition={{
              duration: 8,
              repeat: Infinity,
              ease: "linear",
            }}
          >
            💕
          </motion.div>
        ))}
        {[...Array(8)].map((_, i) => (
          <motion.div
            key={`hk-${i}`}
            className="absolute opacity-10"
            style={{
              left: `${Math.random() * 90 + 5}%`,
              top: `${Math.random() * 90 + 5}%`,
              fontSize: `${Math.random() * 30 + 20}px`,
            }}
            animate={{
              y: [0, -30, 0],
              x: [0, 10, -10, 0],
              rotate: [0, 5, -5, 0],
            }}
            transition={{
              duration: 12,
              repeat: Infinity,
              ease: "easeInOut",
            }}
          >
            🐱
          </motion.div>
        ))}
      </div>

      <motion.div
        initial={{ scale: 0.8, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        transition={{ duration: 0.5 }}
        className="bg-white/90 backdrop-blur-sm rounded-xl p-8 shadow-2xl kawaii-shadow max-w-md w-full mx-4 sparkle"
      >
        <motion.div
          className="text-center mb-8"
          initial={{ y: -20 }}
          animate={{ y: 0 }}
          transition={{ delay: 0.2 }}
        >
          <motion.div
            className="text-6xl mb-4"
            animate={{ rotate: [0, 5, -5, 0] }}
            transition={{ duration: 2, repeat: Infinity }}
          >
            🐱
          </motion.div>
          <h1 className="text-4xl font-bold text-hello-kitty-deep-pink mb-2 font-kawaii">
            Szerelmes Bejelentkezés
          </h1>
          <p className="text-hello-kitty-hot-pink mb-8 font-cute">
            Lépj be a szerelem világába! 💕
          </p>
        </motion.div>

        <form onSubmit={handleSubmit} className="space-y-6">
          <motion.div
            initial={{ x: -20, opacity: 0 }}
            animate={{ x: 0, opacity: 1 }}
            transition={{ delay: 0.3 }}
          >
            <label className="block text-hello-kitty-deep-pink font-semibold mb-2 font-cute">
              Felhasználónév
            </label>
            <input
              type="text"
              value={credentials.username}
              onChange={(e) => setCredentials({...credentials, username: e.target.value})}
              className="w-full px-4 py-3 rounded-lg border-2 border-hello-kitty-pink focus:border-hello-kitty-hot-pink focus:outline-none transition-colors font-cute"
              placeholder="Add meg a neved..."
              required
            />
          </motion.div>

          <motion.div
            initial={{ x: -20, opacity: 0 }}
            animate={{ x: 0, opacity: 1 }}
            transition={{ delay: 0.4 }}
          >
            <label className="block text-hello-kitty-deep-pink font-semibold mb-2 font-cute">
              Jelszó 🔐
            </label>
            <input
              type="password"
              name="password"
              value={credentials.password}
              onChange={handleInputChange}
              className="w-full px-4 py-3 rounded-lg border-2 border-hello-kitty-pink focus:border-hello-kitty-hot-pink focus:outline-none transition-colors bg-hello-kitty-white/80"
              placeholder="A titkos jelszavad..."
              required
            />
          </motion.div>

          {error && (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              className="text-hello-kitty-red text-center font-cute bg-hello-kitty-light-pink/50 p-3 rounded-lg"
            >
              {error}
            </motion.div>
          )}

          <motion.button
            type="submit"
            disabled={isLoading}
            className="w-full bg-gradient-to-r from-hello-kitty-hot-pink to-hello-kitty-deep-pink text-white py-3 px-6 rounded-lg font-bold text-lg kawaii-shadow hover:shadow-lg transition-all duration-300 disabled:opacity-50 font-cute hello-kitty-bow"
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
            initial={{ y: 20, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            transition={{ delay: 0.5 }}
          >
            {isLoading ? (
              <motion.div
                animate={{ rotate: 360 }}
                transition={{ duration: 1, repeat: Infinity, ease: "linear" }}
                className="inline-block"
              >
                🌸
              </motion.div>
            ) : (
              'Bejelentkezés Szeretettel 💕'
            )}
          </motion.button>
        </form>

        <motion.div
          className="text-center mt-6 text-hello-kitty-hot-pink font-cute"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ delay: 0.6 }}
        >
          <p className="text-sm">Szeretettel készítve a különleges valakimnek 💖</p>
        </motion.div>
      </motion.div>
    </div>
  );
};

export default Login;
