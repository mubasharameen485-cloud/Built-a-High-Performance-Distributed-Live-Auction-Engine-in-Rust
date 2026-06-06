// src/lib/chatSocket.js
"use client";

import { useEffect, useRef, useState } from "react";
import { useAuth } from "../context/AuthContext";

const WS_URL = "ws://localhost:3000/ws";

export function useChatSocket() {
  const { user } = useAuth();
  const ws = useRef(null);
  const [liveChats, setLiveChats] = useState([]); 

  useEffect(() => {
    if (!user) return;

    ws.current = new WebSocket(WS_URL);

    ws.current.onmessage = (event) => {
      const data = JSON.parse(event.data);
      
      // SIRF CHAT WALE MESSAGES PAKRO
      if (data.msg_type === "chat_update") {
        setLiveChats((prev) => [
          ...prev, 
          { sender: data.sender, content: data.content }
        ]);
      }
    };

    return () => {
      if (ws.current) ws.current.close();
    };
  }, [user]);

  // Chat bhejne ka function
  const sendChat = (itemId, content) => {
    if (ws.current) {
      ws.current.send(JSON.stringify({ 
        msg_type: "chat", 
        item_id: parseInt(itemId), 
        sender: user.username, 
        content 
      }));
    }
  };

  return { liveChats, setLiveChats, sendChat };
}