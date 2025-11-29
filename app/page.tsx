"use client";

import Button from "@mui/material/Button";
import { useEffect, useState } from "react";
import init, { initThreadPool, run_day } from "@/aoc2025_wasm/pkg/aoc2025_wasm";
import DaySelector from "./day_selector";
import PuzzleInput from "./puzzle_input";
import PuzzleOutput from "./puzzle_output";

function warmup_jit() {
	run_day(1, "0");
}

async function doComputation({
	day,
	inputData,
	setOutputData,
	setTimingData,
}: {
	day: number;
	inputData: string;
	setOutputData: (data: string) => void;
	setTimingData: (data: number | null) => void;
}) {
	setOutputData("");
	setTimingData(null);

	// Should realy put this in a webworker, but
	// webworker + wasm = pain.
	const [ans, timing] = await new Promise<[string, number]>((resolve, _) => {
		console.log("doing work")
		const start = performance.now();
		const ans = run_day(day, inputData);
		const end = performance.now();
		console.log("done work")
		resolve([ans, end - start]);
	});

	console.log("reacting")
	setOutputData(ans);
	setTimingData(timing);

	// setOutputData(ans);
	// setTimingData(end - start);
}

export default function Home() {
	const [outputData, setOutputData] = useState("");
	const [inputData, setInputData] = useState("");
	const [webAssemblyReady, setWebAssemblyReady] = useState(false);
	const [day, setDay] = useState(1);
	const [timingData, setTimingData] = useState<number | null>(null);

	useEffect(() => {
		if (typeof window !== "undefined" && !webAssemblyReady) {
			init().then(() => {
				initThreadPool(navigator.hardwareConcurrency).then(() => {
					warmup_jit();
					setWebAssemblyReady(true);
				});
			});
		}
	}, [webAssemblyReady]);

	return (
		<div className="w-4/5 max-w-[1000px]">
			<DaySelector day={day} setDay={setDay} />
			<PuzzleInput inputData={inputData} setInputData={setInputData} />
			<div className="m-2 w-full">
				<Button
					variant="contained"
					onClick={() => {
						doComputation({ day, inputData, setOutputData, setTimingData });
					}}
					disabled={!webAssemblyReady || inputData.length === 0}
				>
					Run solution
				</Button>
			</div>
			<PuzzleOutput output={outputData} timingData={timingData} />
		</div>
	);
}
