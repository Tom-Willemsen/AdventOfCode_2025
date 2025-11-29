import type { Metadata } from "next";
import { Geist_Mono } from "next/font/google";
import Script from "next/script";
import "./globals.css";
import GitHubIcon from "@mui/icons-material/GitHub";
import { NoSsr } from "@mui/material";
import { AppRouterCacheProvider } from "@mui/material-nextjs/v15-appRouter";
import Link from "next/link";

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
				<Script src="enable-threads.js" strategy="beforeInteractive" />
				<div className="flex items-center bg-gradient-to-r from-[#c9040b] via-[#b0040a] to-[#c9040b] w-screen">
					<h1 className="font-mono p-4 font-bold text-center text-l sm:text-xl text-white grow">
						🎄✨🎄 Advent of Code 2025 🎄✨🎄
					</h1>
					<Link
						href={"https://github.com/Tom-Willemsen/AdventOfCode_2025"}
						target="_blank"
					>
						<GitHubIcon
							className="float-right mx-4"
							fontSize="large"
							htmlColor="#ffffff"
						/>
					</Link>
				</div>
				<hr />
				<div className="flex w-screen p-4 place-content-center align-content-center text-align-center">
					<AppRouterCacheProvider options={{ enableCssLayer: true }}>
						<NoSsr>{children}</NoSsr>
					</AppRouterCacheProvider>
				</div>
			</body>
		</html>
	);
}
