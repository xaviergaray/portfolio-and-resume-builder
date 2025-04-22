import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import globalContent from "@/data/globals.json";
import "./globals.css";
import React from "react";
import Navbar from "./components/Navbar";
import BackgroundBlobs from "@/app/components/BackgroundBlobs";
import Footer from "@/app/components/Footer";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: globalContent.title,
  description: globalContent.description,
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
    return (
    <html lang="en">
      <body className={`${geistSans.variable} ${geistMono.variable} antialiased mx-auto mt-[5rem] px-6 py-8 flex flex-col items-center w-[100vw] md:w-[85vw] text-[--var(--maintext-1)]`}>
        <Navbar />
        <BackgroundBlobs count={4} />
        <div className="pb-[6rem]">
          {children}
        </div>
        <Footer />
      </body>
    </html>
);
}
