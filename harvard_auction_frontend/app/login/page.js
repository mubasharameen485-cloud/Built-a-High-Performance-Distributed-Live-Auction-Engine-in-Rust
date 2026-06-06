// src/app/login/page.jsx
"use client";

import { useForm } from "react-hook-form";
import { useState } from "react";
import { loginUser } from "@/lib/api";
import { useAuth } from "@/context/AuthContext";
import Input from "@/components/ui/Input";

export default function LoginPage() {
  const { login } = useAuth(); // Global state ka login function
  const [errorMsg, setErrorMsg] = useState("");
  
  const { register, handleSubmit, formState: { errors, isSubmitting } } = useForm();

  const onSubmit = async (data) => {
    setErrorMsg("");
    try {
      // Backend ko call milao
      const response = await loginUser(data.username, data.password);
      
      // JADOO: Token ko AuthContext ko de do (Wo khud Home page par bhej dega)
      login(response.token); 
    } catch (err) {
      setErrorMsg(err.message);
    }
  };

  return (
    <div className="flex justify-center items-center h-[80vh]">
      <div className="bg-white p-8 rounded-xl shadow-lg w-full max-w-md border border-gray-100">
        <h2 className="text-3xl font-bold text-center text-slate-800 mb-6">Welcome Back</h2>
        
        {errorMsg && (
          <div className="bg-red-100 text-red-600 p-3 rounded-md mb-4 text-sm text-center">
            {errorMsg}
          </div>
        )}

        <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-4">
          <div>
            <Input 
              label="Username" 
              placeholder="Your username" 
              {...register("username", { required: "Username is required" })} 
            />
            {errors.username && <span className="text-red-500 text-xs">{errors.username.message}</span>}
          </div>

          <div>
            <Input 
              label="Password" 
              type="password" 
              placeholder="Your password" 
              {...register("password", { required: "Password is required" })} 
            />
            {errors.password && <span className="text-red-500 text-xs">{errors.password.message}</span>}
          </div>

          <button 
            type="submit" 
            disabled={isSubmitting}
            className="mt-4 bg-orange-500 hover:bg-orange-600 text-white p-3 rounded-md font-bold transition disabled:bg-orange-300"
          >
            {isSubmitting ? "Authenticating..." : "Login"}
          </button>
        </form>
      </div>
    </div>
  );
}