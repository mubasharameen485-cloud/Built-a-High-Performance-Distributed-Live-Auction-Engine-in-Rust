// src/components/ui/Input.jsx
import { forwardRef } from "react";

// JADOO: forwardRef lagana zaroori hai taake React Hook Form isay parh sake
const Input = forwardRef(({ label, type = "text", placeholder, ...rest }, ref) => {
  return (
    <div className="flex flex-col gap-1 mb-4">
      <label className="text-sm font-semibold text-gray-700">{label}</label>
      <input
        type={type}
        placeholder={placeholder}
        ref={ref} // React Hook Form ka reference yahan attach hoga
        {...rest} // Baqi saari cheezein (jaise onChange, name) yahan aayengi
        className="p-3 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-orange-500 transition"
      />
    </div>
  );
});

Input.displayName = "Input"; // Debugging ke liye
export default Input;