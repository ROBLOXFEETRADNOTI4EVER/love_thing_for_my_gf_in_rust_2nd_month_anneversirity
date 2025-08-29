import React, { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

interface MessageCardProps {
  id: number;
  title: string;
  message: string;
  index: number;
}

const MessageCard: React.FC<MessageCardProps> = ({ id, title, message, index }) => {
  const [isExpanded, setIsExpanded] = useState(false);

  const cardVariants = {
    hidden: { opacity: 0, y: 50, scale: 0.9 },
    visible: { 
      opacity: 1, 
      y: 0, 
      scale: 1,
      transition: { 
        delay: index * 0.1,
        duration: 0.5,
        type: "spring",
        stiffness: 100
      }
    }
  };

  const contentVariants = {
    collapsed: { height: 0, opacity: 0 },
    expanded: { 
      height: "auto", 
      opacity: 1,
      transition: { duration: 0.3, ease: "easeInOut" }
    }
  };

  return (
    <motion.div
      variants={cardVariants}
      initial="hidden"
      whileInView="visible"
      viewport={{ once: true, margin: "-50px" }}
      className="mb-6 mx-4"
    >
      <motion.div
        className="bg-white/95 backdrop-blur-sm rounded-3xl overflow-hidden kawaii-shadow cursor-pointer"
        whileHover={{ scale: 1.02, y: -5 }}
        whileTap={{ scale: 0.98 }}
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <div className="p-6 bg-gradient-to-r from-hello-kitty-soft-pink to-hello-kitty-pastel-pink">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <motion.div
                className="w-12 h-12 bg-hello-kitty-hot-pink rounded-full flex items-center justify-center text-white font-bold text-lg"
                animate={{ rotate: isExpanded ? 180 : 0 }}
                transition={{ duration: 0.3 }}
              >
                {id}
              </motion.div>
              <div>
                <h3 className="text-hello-kitty-deep-pink font-bold text-lg font-kawaii">
                  {title}
                </h3>
                <p className="text-hello-kitty-hot-pink text-sm font-cute">
                  Koppints az édes üzenetért 💕
                </p>
              </div>
            </div>
            <motion.div
              animate={{ rotate: isExpanded ? 180 : 0 }}
              transition={{ duration: 0.3 }}
              className="text-2xl"
            >
              {isExpanded ? '🔓' : '💌'}
            </motion.div>
          </div>
        </div>

        <AnimatePresence>
          {isExpanded && (
            <motion.div
              variants={contentVariants}
              initial="collapsed"
              animate="expanded"
              exit="collapsed"
              className="overflow-hidden"
            >
              <div className="p-6 bg-gradient-to-b from-hello-kitty-white to-hello-kitty-light-pink">
                <motion.div
                  initial={{ opacity: 0, y: 20 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: 0.1 }}
                  className="relative"
                >
                  <div className="absolute -top-2 -left-2 text-hello-kitty-pink opacity-50">
                    <motion.span
                      animate={{ rotate: [0, 10, -10, 0] }}
                      transition={{ duration: 2, repeat: Infinity }}
                    >
                      🌸
                    </motion.span>
                  </div>
                  <div className="absolute -top-2 -right-2 text-hello-kitty-pink opacity-50">
                    <motion.span
                      animate={{ rotate: [0, -10, 10, 0] }}
                      transition={{ duration: 2.5, repeat: Infinity }}
                    >
                      💖
                    </motion.span>
                  </div>

                  <div className="bg-hello-kitty-white/80 rounded-2xl p-4 border-2 border-hello-kitty-pink">
                    <p className="text-hello-kitty-deep-pink text-lg leading-relaxed font-cute text-center">
                      "{message}"
                    </p>
                  </div>

                  <div className="flex justify-center mt-4 space-x-2">
                    {['💕', '🎀', '💕', '🥰'].map((emoji, i) => (
                      <motion.span
                        key={i}
                        animate={{ 
                          y: [0, -5, 0],
                          scale: [1, 1.1, 1]
                        }}
                        transition={{ 
                          duration: 1.5,
                          repeat: Infinity,
                          delay: i * 0.2
                        }}
                        className="text-hello-kitty-hot-pink"
                      >
                        {emoji}
                      </motion.span>
                    ))}
                  </div>
                </motion.div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </motion.div>
    </motion.div>
  );
};

export default MessageCard;
