// src/components/auction/AuctionCard.jsx
"use client";

import { useState } from "react";
import { useAuth } from "@/context/AuthContext";

export default function AuctionCard({ item, sendLiveBid }) {
  const { user } = useAuth();
  const [bidAmount, setBidAmount] = useState("");

  const handleBid = () => {
    if (!user) {
      alert("Boli lagane ke liye pehle Login karein!");
      return;
    }
    
    // THE FIX: item.id ki jagah item.item_id likhna hai!
    if (!item || !item.item_id) {
      alert("Abhi koi item auction ke liye majood nahi hai! Admin ka intezar karein.");
      return;
    }
    
    const amount = parseFloat(bidAmount);
    if (!amount || isNaN(amount)) {
      alert("Sahi qeemat likhein!");
      return;
    }
    
    if (amount <= item.current_price) {
      alert(`Error! Boli ki qeemat $${item.current_price} se ZYADA honi chahiye.`);
      return;
    }
    
    // THE FIX: Yahan bhi item.item_id aayega!
    sendLiveBid(item.item_id, amount);
    setBidAmount(""); 
  };

  return (
    <div className="bg-white p-6 rounded-xl shadow-lg border border-gray-100">
      {item.image_url && (
        <div className="mb-4 flex justify-center">
          {/* Image URL backend se aa rahi hai */}
          <img src={`http://localhost:3000${item.image_url}`} alt={item.name} className="max-h-48 rounded-md object-cover" />
        </div>
      )}
      
      <h3 className="text-xl font-bold text-slate-800 mb-2">{item.name}</h3>
      <p className="text-gray-600 text-sm mb-4">{item.description}</p>

      <div className="bg-slate-50 p-3 rounded-md mb-4 flex justify-between items-center">
        <span className="text-sm font-semibold text-gray-500">Current Price:</span>
        <span className="text-2xl font-black text-green-600">${item.current_price}</span>
      </div>

      <div className="flex gap-2">
        <input
          type="number"
          value={bidAmount}
          onChange={(e) => setBidAmount(e.target.value)}
          placeholder={`Min $${item.current_price + 1}`}
          className="flex-1 p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-500"
        />
        <button
          onClick={handleBid}
          className="bg-orange-500 hover:bg-orange-600 text-white px-4 py-2 rounded-md font-bold transition"
        >
          Bid Now
        </button>
      </div>
    </div>
  );
}