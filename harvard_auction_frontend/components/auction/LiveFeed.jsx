// src/components/auction/LiveFeed.jsx
"use client";

export default function LiveFeed({ events }) {
  return (
    <div className="bg-slate-900 rounded-xl p-4 h-[500px] flex flex-col shadow-lg">
      
      <h3 className="text-orange-400 font-bold text-lg mb-4 flex items-center gap-2">
        <span className="w-3 h-3 bg-red-500 rounded-full animate-pulse"></span>
        Live Activity Feed
      </h3>
      
      <div className="flex-1 overflow-y-auto pr-2 space-y-2">
        {events.length === 0 ? (
          <p className="text-gray-500 text-sm italic text-center mt-10">No live activity yet...</p>
        ) : (
          events.map((event, index) => (
            <div 
              key={index} 
              className="bg-slate-800 text-green-400 p-3 rounded-md text-sm border-l-4 border-green-500 shadow-sm animate-fade-in-down"
            >
              {event}
            </div>
          ))
        )}
      </div>
    </div>
  );
}