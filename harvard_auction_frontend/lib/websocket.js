// src/lib/websocket.js
"use client";

import { useEffect, useRef, useState } from "react";
import { useAuth } from "../context/AuthContext";

const WS_URL = "ws://localhost:3000/ws";

export function useAuctionSocket() {
  const { user } = useAuth();
  const ws = useRef(null);
  
  // Ab hamare paas 2 alag lists hain
  const [liveBids, setLiveBids] = useState([]); 
  const [liveChats, setLiveChats] = useState([]); 
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    if (!user) return;
    ws.current = new WebSocket(WS_URL);
    ws.current.onopen = () => setIsConnected(true);

    ws.current.onmessage = (event) => {
      const data = JSON.parse(event.data);
      
      // Agar Bid aayi hai
      if (data.msg_type === "bid_update") {
        setLiveBids((prev) => [{ amount: data.amount, time: new Date().toLocaleTimeString() }, ...prev]);
      } 
      // Agar Chat aayi hai
      else if (data.msg_type === "chat_update") {
        setLiveChats((prev) => [...prev, { sender: data.sender, content: data.content }]);
      }
    };

    ws.current.onclose = () => setIsConnected(false);
    return () => { if (ws.current) ws.current.close(); };
  }, [user]);

  // Boli bhejna
  const sendLiveBid = (itemId, amount) => {
    if (ws.current && isConnected) {
      ws.current.send(JSON.stringify({ msg_type: "bid", item_id: parseInt(itemId), user_id: parseInt(user.sub), amount: parseFloat(amount) }));
    }
  };

  // Chat bhejna
  const sendChat = (itemId, content) => {
    if (ws.current && isConnected) {
      ws.current.send(JSON.stringify({ msg_type: "chat", item_id: parseInt(itemId), sender: user.username, content }));
    }
  };

  return { liveBids, setLiveBids, liveChats, setLiveChats, isConnected, sendLiveBid, sendChat };
}