// src/app/layout.jsx
import "./globals.css";
import { AuthProvider } from "@/context/AuthContext";
import Navbar from "@/components/ui/Navbar"; // Naya import

export const metadata = {
  title: "Harvard Live Auction",
  description: "Real-time Bidding Engine",
};

export default function RootLayout({ children }) {
  return (
    <html lang="en">
      <body className="bg-slate-50 min-h-screen">
        <AuthProvider>
          {/* Navbar ko yahan laga diya taake har page par rahe */}
          <Navbar /> 
          
          {/* Asal Page ka content yahan aayega */}
          <main className="max-w-6xl mx-auto p-4">
            {children}
          </main>
        </AuthProvider>
      </body>
    </html>
  );
}