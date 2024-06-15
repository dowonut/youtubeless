import React from "react";
import ReactDOM from "react-dom/client";
import Root from "./routes/Root.tsx";
import "./index.css";
import "./variables.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Feed from "./components/Feed/Feed.tsx";
import Search from "./components/Search/Search.tsx";
import Profile from "./components/Profile/Profile.tsx";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root></Root>,
    children: [
      { path: "/", element: <Feed></Feed> },
      { path: "/search", element: <Search></Search> },
      { path: "/profile", element: <Profile></Profile> },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <RouterProvider router={router}></RouterProvider>
  </React.StrictMode>
);
