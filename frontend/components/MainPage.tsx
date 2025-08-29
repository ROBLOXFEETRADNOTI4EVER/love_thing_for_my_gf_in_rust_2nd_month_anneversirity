import React, { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import { useAuth } from '../contexts/AuthContext';
import { ParsedMessage } from '../types/auth';
import MessageCard from './MessageCard';

const MainPage: React.FC = () => {
  const { logout, user } = useAuth();
  const [messages, setMessages] = useState<ParsedMessage[]>([]);
  const [todayMessage, setTodayMessage] = useState<string>('');

  const parseMessages = (content: string): ParsedMessage[] => {
    const messages: ParsedMessage[] = [];
    const messageBlocks = content.split(/\[\s*\d+/).filter(block => block.trim());
    
    messageBlocks.forEach((block, index) => {
      const trimmedBlock = block.trim();
      if (!trimmedBlock) return;
      
      const titleMatch = trimmedBlock.match(/^\s*\(([^)]+)\)/);
      const title = titleMatch ? titleMatch[1] : `Üzenet ${index + 1}`;
      
      const contentAfterTitle = trimmedBlock.replace(/^\s*\([^)]+\)\s*/, '');
      const messageContent = contentAfterTitle.replace(/\]\s*$/, '').trim();
      
      if (messageContent) {
        messages.push({
          id: index + 1,
          title: title,
          content: messageContent
        });
      }
    });
    
    return messages;
  };

  useEffect(() => {
    let isMounted = true;
    
    const loadMessages = async () => {
      const token = localStorage.getItem('love_daily_token');
      if (!token || !isMounted) return;

      try {
        const [allMsgsResponse, dailyMsgResponse] = await Promise.all([
          fetch('/api/get_all_msgs', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({ token })
          }),
          fetch('/api/daily_messages', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({ token })
          })
        ]);

        if (allMsgsResponse.ok && isMounted) {
          const allMsgsData = await allMsgsResponse.json();
          const parsedData = JSON.parse(allMsgsData);
          if (parsedData.Lovemsg) {
            const parsedMessages = parseMessages(parsedData.Lovemsg);
            setMessages(parsedMessages);
          }
        }

        if (dailyMsgResponse.ok && isMounted) {
          const dailyMsgData = await dailyMsgResponse.json();
          const parsedDaily = JSON.parse(dailyMsgData);
          if (parsedDaily.Lovemsg && parsedDaily.Lovemsg !== "No new message yet") {
            setTodayMessage(parsedDaily.Lovemsg);
          }
        }
      } catch (error) {
        console.error('Error loading messages:', error);
        if (isMounted) {
          const fallbackMessages: ParsedMessage[] = [
            { id: 1, title: "Szerelmes üzenet", content: "Örökkön örökké szeretni foglak szerelmem." },
            { id: 2, title: "Szerelmes üzenet", content: "Te vagy az egyetlen ember akivel megosztok mindent." },
            { id: 3, title: "Szerelmes üzenet", content: "Melletted Sose félek önmagam lenni." },
            { id: 4, title: "Szerelmes üzenet", content: "Az érintésed a legjobb." },
            { id: 5, title: "Szerelmes üzenet", content: "Átakarlak ölelni nagyon." }
          ];
          setMessages(fallbackMessages);
          setTodayMessage(fallbackMessages[0].content);
        }
      }
    };

    loadMessages();
    
    return () => {
      isMounted = false;
    };
  }, []);

  return (
    <div className="min-h-screen bg-gradient-to-br from-hello-kitty-light-pink via-hello-kitty-pink to-hello-kitty-soft-pink">
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        {[...Array(12)].map((_, i) => (
          <motion.div
            key={i}
            className="absolute text-hello-kitty-pink opacity-15"
            style={{
              left: `${Math.random() * 100}%`,
              top: `${Math.random() * 100}%`,
              fontSize: `${Math.random() * 10 + 8}px`,
            }}
            animate={{
              y: [0, -25, 0],
              rotate: [0, 180, 360],
              scale: [0.7, 1, 0.7],
            }}
            transition={{
              duration: 10,
              repeat: Infinity,
              ease: "linear",
            }}
          >
            💕
          </motion.div>
        ))}
        {[...Array(6)].map((_, i) => (
          <motion.div
            key={`hk-${i}`}
            className="absolute opacity-8"
            style={{
              left: `${Math.random() * 90 + 5}%`,
              top: `${Math.random() * 90 + 5}%`,
              fontSize: `${Math.random() * 25 + 15}px`,
            }}
            animate={{
              y: [0, -20, 0],
              x: [0, 15, -15, 0],
              rotate: [0, 3, -3, 0],
            }}
            transition={{
              duration: 15,
              repeat: Infinity,
              ease: "easeInOut",
            }}
          >
            🐱
          </motion.div>
        ))}
        {[...Array(3)].map((_, i) => (
          <motion.div
            key={`hk-img-${i}`}
            className="absolute opacity-8"
            style={{
              left: `${Math.random() * 90 + 5}%`,
              top: `${Math.random() * 90 + 5}%`,
            }}
            animate={{
              y: [0, -25, 0],
              x: [0, 20, -20, 0],
              rotate: [0, 5, -5, 0],
            }}
            transition={{
              duration: 20,
              repeat: Infinity,
              ease: "easeInOut",
            }}
          >
            <img src="https://www.svgrepo.com/show/443128/brand-hello-kitty.svg" alt="Hello Kitty" className="w-8 h-8" />
          </motion.div>
        ))}
        {[...Array(4)].map((_, i) => (
          <motion.div
            key={`dd-${i}`}
            className="absolute opacity-6"
            style={{
              left: `${Math.random() * 85 + 10}%`,
              top: `${Math.random() * 85 + 10}%`,
              fontSize: `${Math.random() * 20 + 12}px`,
            }}
            animate={{
              y: [0, -18, 0],
              x: [0, 8, -8, 0],
              rotate: [0, 2, -2, 0],
            }}
            transition={{
              duration: 18,
              repeat: Infinity,
              ease: "easeInOut",
            }}
          >
           ❤️ <img src="https://www.geekcals.com/wp-content/uploads/2015/08/hellokitty.png" alt="Hello Kitty" className="w-8 h-8" />
          </motion.div>
        ))}
      </div>

      <div className="relative z-10">
        <motion.header
          initial={{ y: -50, opacity: 0 }}
          animate={{ y: 0, opacity: 1 }}
          className="bg-white/80 backdrop-blur-sm shadow-lg kawaii-shadow"
        >
          <div className="max-w-4xl mx-auto px-6 py-4 flex justify-between items-center">
            <div className="flex items-center space-x-3">
              <motion.div
                animate={{ rotate: [0, 10, -10, 0] }}
                transition={{ duration: 2, repeat: Infinity }}
                className="text-3xl"
              >
                🐱
                
              </motion.div>
              <div>
                <h1 className="text-4xl font-bold text-hello-kitty-deep-pink mb-4 font-kawaii text-center">
                  Mai Szerelmes Üzenet 💕
                </h1>
                <p className="text-hello-kitty-hot-pink text-sm font-cute">
                  Üdvözöllek, {user?.username}! 💖
                </p>
              </div>
            </div>
            <motion.button
              onClick={logout}
              className="bg-hello-kitty-hot-pink text-white px-4 py-2 rounded-2xl font-cute kawaii-shadow hover:shadow-lg transition-all"
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
            >
              Kijelentkezés 🚪
            </motion.button>
          </div>
        </motion.header>

        <motion.div
          initial={{ scale: 0.9, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          transition={{ delay: 0.3 }}
          className="max-w-2xl mx-auto px-6 py-12"
        >
          <div className="text-center mb-12">
            <motion.div
              animate={{ 
                y: [0, -10, 0],
                scale: [1, 1.05, 1]
              }}
              transition={{ 
                duration: 3,
                repeat: Infinity,
                ease: "easeInOut"
              }}
              className="bg-white/95 backdrop-blur-sm rounded-3xl p-8 kawaii-shadow sparkle mb-8"
            >
              <div className="flex justify-center mb-4">
                <motion.div
                  animate={{ rotate: [0, 5, -5, 0] }}
                  transition={{ duration: 2, repeat: Infinity }}
                  className="text-4xl"
                >
                  💌
                </motion.div>
              </div>
              <h2 className="text-hello-kitty-deep-pink font-bold text-xl mb-4 font-kawaii">
                Mai Szerelmes Üzenet
              </h2>
              <p className="text-hello-kitty-hot-pink text-lg font-cute mb-6">
                "{todayMessage || "Betöltés..."}"
              </p>
              <div className="flex justify-center mt-4 space-x-2">
                {['💕', '🌸', '💕'].map((emoji, i) => (
                  <motion.span
                    key={i}
                    animate={{ 
                      y: [0, -5, 0],
                      rotate: [0, 10, -10, 0]
                    }}
                    transition={{ 
                      duration: 2,
                      repeat: Infinity,
                      delay: i * 0.3
                    }}
                    className="text-hello-kitty-pink text-xl"
                  >
                    {emoji}
                  </motion.span>
                ))}
              </div>
            </motion.div>

            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ delay: 0.6 }}
            >
              <h3 className="text-hello-kitty-deep-pink font-bold text-lg mb-6 font-kawaii text-center">
                Görgess le az összes szerelmes üzenetért! 💖 
              </h3>
              <motion.div
                animate={{ y: [0, 10, 0] }}
                transition={{ duration: 1.5, repeat: Infinity }}
                className="text-hello-kitty-hot-pink text-2xl flex justify-center"
              >
                <svg width="10%" height="10%" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M9 3.8C9 3.51997 9 3.37996 9.0545 3.273C9.10243 3.17892 9.17892 3.10243 9.273 3.0545C9.37996 3 9.51997 3 9.8 3H14.2C14.48 3 14.62 3 14.727 3.0545C14.8211 3.10243 14.8976 3.17892 14.9455 3.273C15 3.37996 15 3.51997 15 3.8V14H19L12 21L5 14H9V3.8Z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                </svg>
              </motion.div>
            </motion.div>
          </div>

          <div className="space-y-6">
            {messages.map((message, index) => (
              <MessageCard
                key={index}
                id={message.id}
                title={message.title}
                message={message.content}
                index={index}
              />
            ))}
          </div>

          <motion.div
            initial={{ opacity: 0, y: 50 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            className="text-center mt-16 p-8 bg-white/80 backdrop-blur-sm rounded-3xl kawaii-shadow"
          >
            <div className="text-4xl mb-4">🎀</div>
            <p className="text-hello-kitty-deep-pink font-bold text-lg font-kawaii mb-2">
              Elérted a végét!
            </p>
            <p className="text-hello-kitty-hot-pink text-center font-cute">
              Minden üzenet egy kis szerelem 💖
            </p>
          </motion.div>
        </motion.div>
      </div>
    </div>
  );
};

export default MainPage;
