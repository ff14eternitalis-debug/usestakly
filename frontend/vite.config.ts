import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  const siteOrigin = (env.VITE_SITE_ORIGIN || "http://127.0.0.1:5173").replace(/\/+$/, "");

  return {
    plugins: [
      react(),
      tailwindcss(),
      {
        name: "inject-site-origin-html",
        transformIndexHtml(html) {
          return html.replaceAll("%SITE_ORIGIN%", siteOrigin);
        }
      }
    ],
    server: {
      port: 5173
    }
  };
});

