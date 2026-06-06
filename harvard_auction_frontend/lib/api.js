// src/lib/api.js

const API_BASE_URL = "http://localhost:3000"; // Hamara Axum API Gateway

// Ek chota sa helper function jo har request mein Token (ID Card) lagayega
async function fetchWithAuth(endpoint, options = {}) {
  const token = localStorage.getItem("token"); // Browser ki memory se token nikalo
  
  const headers = {
    "Content-Type": "application/json",
    ...(token && { Authorization: `Bearer ${token}` }), // Agar token hai toh header mein daal do
    ...options.headers,
  };

  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    ...options,
    headers,
  });

  const data = await response.json();
  
  if (!response.ok) {
    throw new Error(data.message || "API Request Failed");
  }
  return data;
}

// 1. Signup API Call
export async function signupUser(username, password) {
  return fetchWithAuth("/signup", {
    method: "POST",
    body: JSON.stringify({ username, password }),
  });
}

// 2. Login API Call
export async function loginUser(username, password) {
  return fetchWithAuth("/login", {
    method: "POST",
    body: JSON.stringify({ username, password }),
  });
}

// Admin: Naya Item Banao (Image URL ke sath)
export async function createAuctionItem(name, description, startingPrice, imageUrl) {
  return fetchWithAuth("/items", {
    method: "POST",
    body: JSON.stringify({ 
      name, 
      description, 
      starting_price: parseFloat(startingPrice),
      image_url: imageUrl || null
    }),
  });
}

// User: Page refresh par Active Item lao
export async function getActiveItem() {
  return fetchWithAuth("/items/active", { method: "GET" });
}

// User: Item ki History (Bids aur Chats) lao
export async function getItemHistory(itemId) {
  return fetchWithAuth(`/items/${itemId}/history`, { method: "GET" });
}
