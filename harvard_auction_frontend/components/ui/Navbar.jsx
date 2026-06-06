// src/components/ui/Navbar.jsx
"use client";

import Link from "next/link";
import { useAuth } from "@/context/AuthContext";

export default function Navbar() {
  const { user, logout } = useAuth(); // Global state se user aur logout function nikala

  return (
    <nav className="bg-slate-900 text-white p-4 shadow-md">
      <div className="max-w-6xl mx-auto flex justify-between items-center">
        
        {/* Logo / Brand Name */}
        <Link href="/" className="text-2xl font-bold tracking-wider text-orange-400">
          Harvard Auction
        </Link>

        {/* Navigation Links */}
        <div className="flex gap-4 items-center">
          {user ? (
            <>
              {/* Agar user login hai */}
              <span className="text-gray-300">
                Welcome, <strong className="text-white">{user.username}</strong> 
                {user.role === "admin" && <span className="text-red-400 ml-1">(Admin)</span>}
              </span>
              <button 
                onClick={logout} 
                className="bg-red-600 hover:bg-red-700 px-4 py-2 rounded-md font-semibold transition"
              >
                Logout
              </button>
            </>
          ) : (
            <>
              {/* Agar user login nahi hai */}
              <Link href="/login" className="hover:text-orange-400 transition">
                Login
              </Link>
              <Link href="/signup" className="bg-orange-500 hover:bg-orange-600 px-4 py-2 rounded-md font-semibold transition">
                Sign Up
              </Link>
            </>
          )}
        </div>
      </div>
    </nav>
  );
}