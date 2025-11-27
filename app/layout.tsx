import type { Metadata } from "next";
import { Geist_Mono } from "next/font/google";
import Script from "next/script";
import "./globals.css";
import { AppRouterCacheProvider } from "@mui/material-nextjs/v15-appRouter";

const geistMono = Geist_Mono({
	variable: "--font-geist-mono",
	subsets: ["latin"],
});

export const metadata: Metadata = {
	title: "AoC 2025",
	description: "Advent of Code 2025",
};

export default function RootLayout({
	children,
}: Readonly<{
	children: React.ReactNode;
}>) {
	return (
		<html lang="en">
			<body className={`${geistMono.variable} antialiased`}>
				<Script src="enable-threads.js" />
				<div className="flex place-content-center bg-gradient-to-r from-[#c9040b] via-[#b0040a] to-[#c9040b]">
					<h1 className="font-mono p-4 font-bold text-align-center text-l sm:text-xl text-white">
						🎄✨🎄 Advent of Code 2025 🎄✨🎄
					</h1>
				</div>
				<hr />
				<div className="flex w-screen p-4 place-content-center align-content-center text-align-center">
					<AppRouterCacheProvider options={{ enableCssLayer: true }}>
						{children}
					</AppRouterCacheProvider>
				</div>
			</body>
		</html>
	);
}
