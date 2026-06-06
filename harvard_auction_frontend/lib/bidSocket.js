// src/lib/bidSocket.js
"use client";

import { useEffect, useRef, useState } from "react";
import { useAuth } from "../context/AuthContext";

const WS_URL = "ws://localhost:3000/ws";

export function useBidSocket() {
  const { user } = useAuth();
  const ws = useRef(null);
  const [liveBids, setLiveBids] = useState([]); 
  const [isBidConnected, setIsBidConnected] = useState(false);

  useEffect(() => {
    if (!user) return;

    ws.current = new WebSocket(WS_URL);

    ws.current.onopen = () => {
      console.log("Bid Socket Connected!");
      setIsBidConnected(true);
    };

    ws.current.onmessage = (event) => {
      const data = JSON.parse(event.data);
      
      // SIRF BID WALE MESSAGES PAKRO
      if (data.msg_type === "bid_update") {
        console.log("Nayi Boli Aayi:", data);
        setLiveBids((prev) => [
          { amount: data.amount, user_id: data.user_id, time: new Date().toLocaleTimeString() }, 
          ...prev
        ]);
      }
    };

    ws.current.onclose = () => setIsBidConnected(false);

    return () => {
      if (ws.current) ws.current.close();
    };
  }, [user]);

  // Boli bhejne ka function
  const sendLiveBid = (itemId, amount) => {
    if (ws.current && isBidConnected) {
      ws.current.send(JSON.stringify({ 
        msg_type: "bid", 
        item_id: parseInt(itemId), 
        user_id: parseInt(user.sub), 
        amount: parseFloat(amount) 
      }));
    } else {
      alert("Bid Server se connection toota hua hai!");
    }
  };

  return { liveBids, setLiveBids, isBidConnected, sendLiveBid };
}