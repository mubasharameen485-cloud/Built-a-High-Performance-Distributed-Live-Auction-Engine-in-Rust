// src/context/AuthContext.js
"use client"; // Next.js ko batana ke yeh client-side code hai

import { createContext, useState, useEffect, useContext } from "react";
import { useRouter } from "next/navigation";

// Context banana
const AuthContext = createContext();

export function AuthProvider({ children }) {
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(true);
  const router = useRouter();

  // JADOO: Token ke andar se data (Claims) nikalne ka function
  const decodeToken = (token) => {
    try {
      const payload = token.split(".")[1];
      const decoded = JSON.parse(atob(payload)); // Base64 se JSON mein badalna
      return decoded;
    } catch (e) {
      return null;
    }
  };

  // Jab bhi app khulegi, yeh check karega ke kya pehle se login hai?
  useEffect(() => {
    const token = localStorage.getItem("token");
    if (token) {
      const decodedUser = decodeToken(token);
      if (decodedUser && decodedUser.exp * 1000 > Date.now()) {
        setUser({ token, ...decodedUser }); // Token theek hai, user set kar do
      } else {
        localStorage.removeItem("token"); // Token expire ho gaya
      }
    }
    setLoading(false);
  }, []);

  // Login Function (Jo api.js se data le kar state update karega)
  const login = (token) => {
    localStorage.setItem("token", token);
    const decodedUser = decodeToken(token);
    setUser({ token, ...decodedUser });
    router.push("/"); // Login ke baad Home page par bhej do
  };

  // Logout Function
  const logout = () => {
    localStorage.removeItem("token");
    setUser(null);
    router.push("/login"); // Logout ke baad Login page par bhej do
  };

  return (
    <AuthContext.Provider value={{ user, login, logout, loading }}>
      {!loading && children}
    </AuthContext.Provider>
  );
}

// Ek custom hook taake baqi pages asaani se 'useAuth()' likh sakein
export const useAuth = () => useContext(AuthContext);