// src/app/admin/page.jsx
"use client";

import { useForm } from "react-hook-form";
import { useState } from "react";
import { createAuctionItem } from "@/lib/api";
import { useAuth } from "@/context/AuthContext";
import Input from "@/components/ui/Input";

export default function AdminDashboard() {
  const { user } = useAuth();
  const [msg, setMsg] = useState("");
  const { register, handleSubmit, reset, formState: { isSubmitting } } = useForm();

  // Agar user Admin nahi hai, toh usay bahar nikal do
  if (!user || user.role !== "admin") {
    return <div className="text-center mt-20 text-red-500 font-bold text-2xl">Access Denied! You are not an Admin.</div>;
  }

  const onSubmit = async (data) => {
    try {
      await createAuctionItem(data.name, data.description, data.startingPrice, data.imageUrl);
      setMsg("Item successfully added to Live Auction!");
      reset(); // Form khali kar do
    } catch (err) {
      setMsg(`Error: ${err.message}`);
    }
  };

  return (
    <div className="max-w-2xl mx-auto mt-10 bg-white p-8 rounded-xl shadow-lg">
      <h2 className="text-3xl font-bold text-slate-800 mb-6 border-b pb-2">Admin Dashboard</h2>
      <p className="text-gray-600 mb-6">Create a new item for the Live Auction.</p>
      
      {msg && <div className="bg-blue-100 text-blue-700 p-3 rounded-md mb-4">{msg}</div>}

      <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-4">
        <Input label="Item Name" {...register("name", { required: true })} />
        <Input label="Description" {...register("description", { required: true })} />
        <Input label="Starting Price ($)" type="number" {...register("startingPrice", { required: true })} />
        <Input label="Image URL (Optional)" placeholder="e.g., https://example.com/watch.png" {...register("imageUrl")} />

        <button type="submit" disabled={isSubmitting} className="bg-orange-500 text-white p-3 rounded-md font-bold mt-4">
          {isSubmitting ? "Creating..." : "Start Auction"}
        </button>
      </form>
    </div>
  );
}