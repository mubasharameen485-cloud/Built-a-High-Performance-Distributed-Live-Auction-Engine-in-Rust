// src/app/page.jsx
"use client";

import { useState, useEffect } from "react";
import { useAuth } from "@/context/AuthContext";
import { getActiveItem, getItemHistory } from "@/lib/api";
import AuctionCard from "@/components/auction/AuctionCard";
import LiveFeed from "@/components/auction/LiveFeed";

// Naye alag alag sockets import kiye
import { useBidSocket } from "@/lib/bidSocket";
import { useChatSocket } from "@/lib/chatSocket";

export default function HomePage() {
  const { user, loading } = useAuth();
  
  // Dono sockets ko alag alag initialize kiya
  const { liveBids, setLiveBids, isBidConnected, sendLiveBid } = useBidSocket();
  const { liveChats, setLiveChats, sendChat } = useChatSocket();

  const [auctionItem, setAuctionItem] = useState(null);
  const [chatInput, setChatInput] = useState("");
  const [pageLoading, setPageLoading] = useState(true);

  // Page Load hote hi History lana
  useEffect(() => {
    async function loadAuctionData() {
      try {
        const item = await getActiveItem();
        setAuctionItem(item);

        const history = await getItemHistory(item.item_id);
        setLiveBids(history.bids);
        setLiveChats(history.chats);
      } catch (error) {
        console.error("Data load nahi ho saka:", error.message);
      } finally {
        setPageLoading(false);
      }
    }

    if (user) loadAuctionData();
    else if (!loading) setPageLoading(false);
  }, [user, loading, setLiveBids, setLiveChats]);

  // Live Price Update Logic
  useEffect(() => {
    if (liveBids.length > 0 && auctionItem) {
      const latestBid = liveBids[0]; 
      if (latestBid.amount > auctionItem.current_price) {
        setAuctionItem((prev) => ({ ...prev, current_price: latestBid.amount }));
      }
    }
  }, [liveBids]);

  const handleSendChat = (e) => {
    e.preventDefault();
    if (chatInput.trim() !== "" && auctionItem) {
      sendChat(auctionItem.item_id, chatInput);
      setChatInput(""); 
    }
  };

  if (loading || pageLoading) return <div className="text-center mt-20 text-xl font-bold">Loading Engine...</div>;
  if (!user) return <div className="text-center mt-20 text-xl text-gray-600">Live Auction dekhne ke liye Login karein.</div>;
  if (!auctionItem) return <div className="text-center mt-20 text-xl text-gray-600">Abhi koi Auction nahi chal rahi.</div>;

  return (
    <div className="mt-8">
      <div className="mb-8 text-center">
        <h1 className="text-4xl font-black text-slate-800 mb-2">Live Auction Arena</h1>
        <p className="text-gray-500">
          Status: {isBidConnected ? <span className="text-green-500 font-bold">● Live Connected</span> : <span className="text-red-500">Disconnected</span>}
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
        
        {/* LEFT COLUMN: LIVE CHAT BOX */}
        <div className="lg:col-span-1 bg-white rounded-xl shadow-md border border-gray-100 flex flex-col h-[500px]">
          <h3 className="bg-slate-800 text-white p-3 rounded-t-xl font-bold text-center">Live Chat</h3>
          <div className="flex-1 overflow-y-auto p-4 space-y-3 bg-slate-50">
            {liveChats.map((chat, index) => (
              <div key={index} className={`p-2 rounded-lg text-sm ${chat.sender === user.username ? 'bg-blue-100 ml-auto' : 'bg-white border'}`}>
                <strong className="text-xs text-gray-500 block">{chat.sender}</strong>
                <span>{chat.content}</span>
              </div>
            ))}
          </div>
          <form onSubmit={handleSendChat} className="p-3 border-t flex gap-2">
            <input type="text" value={chatInput} onChange={(e) => setChatInput(e.target.value)} placeholder="Say something..." className="flex-1 p-2 text-sm border rounded-md" />
            <button type="submit" className="bg-blue-600 text-white px-3 py-2 rounded-md text-sm font-bold">Send</button>
          </form>
        </div>

        {/* CENTER COLUMN: AUCTION ITEM CARD */}
        <div className="lg:col-span-2">
          <AuctionCard item={auctionItem} sendLiveBid={sendLiveBid} />
        </div>

        {/* RIGHT COLUMN: LIVE BIDS HISTORY */}
        <div className="lg:col-span-1 bg-slate-900 rounded-xl shadow-md flex flex-col h-[500px]">
          <h3 className="text-orange-400 p-3 font-bold text-center border-b border-slate-700">Live Bids</h3>
          <div className="flex-1 overflow-y-auto p-3 space-y-2">
            {liveBids.map((bid, index) => (
              <div key={index} className="bg-slate-800 p-3 rounded-md border-l-4 border-green-500 flex justify-between items-center">
                <span className="text-gray-300 text-xs">User {bid.user_id}</span>
                <span className="text-green-400 font-bold">${bid.amount}</span>
              </div>
            ))}
          </div>
        </div>

      </div>
    </div>
  );
}