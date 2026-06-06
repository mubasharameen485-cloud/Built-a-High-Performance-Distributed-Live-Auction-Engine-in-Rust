// src/app/signup/page.jsx
"use client";

import { useForm } from "react-hook-form";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { signupUser } from "@/lib/api";
import Input from "@/components/ui/Input";

export default function SignupPage() {
  const router = useRouter();
  const [errorMsg, setErrorMsg] = useState("");
  
  // React Hook Form ka jadoo
  const { register, handleSubmit, formState: { errors, isSubmitting } } = useForm();

  const onSubmit = async (data) => {
    setErrorMsg("");
    try {
      // Backend (Axum Gateway) ko call milao
      await signupUser(data.username, data.password);
      alert("Signup Successful! Please login.");
      router.push("/login"); // Kamyabi ke baad login page par bhej do
    } catch (err) {
      setErrorMsg(err.message);
    }
  };

  return (
    <div className="flex justify-center items-center h-[80vh]">
      <div className="bg-white p-8 rounded-xl shadow-lg w-full max-w-md border border-gray-100">
        <h2 className="text-3xl font-bold text-center text-slate-800 mb-6">Join the Auction</h2>
        
        {errorMsg && (
          <div className="bg-red-100 text-red-600 p-3 rounded-md mb-4 text-sm text-center">
            {errorMsg}
          </div>
        )}

        <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-4">
          <div>
            <Input 
              label="Username" 
              placeholder="Enter unique username" 
              {...register("username", { 
                required: "Username is required", 
                minLength: { value: 3, message: "Username must be at least 3 characters" } 
              })} 
            />
            {errors.username && <span className="text-red-500 text-xs">{errors.username.message}</span>}
          </div>

          <div>
            <Input 
              label="Password" 
              type="password" 
              placeholder="Enter strong password" 
              {...register("password", { 
                required: "Password is required", 
                minLength: { value: 6, message: "Password must be at least 6 characters" } 
              })} 
            />
            {errors.password && <span className="text-red-500 text-xs">{errors.password.message}</span>}
          </div>

          <button 
            type="submit" 
            disabled={isSubmitting}
            className="mt-4 bg-slate-900 hover:bg-slate-800 text-white p-3 rounded-md font-bold transition disabled:bg-gray-400"
          >
            {isSubmitting ? "Creating Account..." : "Sign Up"}
          </button>
        </form>
      </div>
    </div>
  );
}