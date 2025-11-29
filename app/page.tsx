"use client";

import Button from "@mui/material/Button";
import CircularProgress from "@mui/material/CircularProgress";
import { useMemo, useState } from "react";
import DaySelector from "./day_selector";
import PuzzleInput from "./puzzle_input";
import PuzzleOutput from "./puzzle_output";

function doComputation({
	day,
	inputData,
	setOutputData,
	setTimingData,
	setBusy,
	worker,
}: {
	day: number;
	inputData: string;
	setOutputData: (data: string) => void;
	setTimingData: (data: number | null) => void;
	setBusy: (data: boolean) => void;
	worker: Worker;
}) {
	setOutputData("");
	setTimingData(null);
	setBusy(true);
	worker.postMessage([day, inputData]);
}

export default function Home() {
	const [outputData, setOutputData] = useState("");
	const [inputData, setInputData] = useState("");
	const [day, setDay] = useState(1);
	const [timingData, setTimingData] = useState<number | null>(null);
	const [busy, setBusy] = useState(false);

	const worker = useMemo<Worker>(() => {
		const worker = new Worker(new URL("webworker.ts", import.meta.url));
		worker.onmessage = (msg) => {
			const [output, timing] = msg.data;
			setOutputData(output);
			setTimingData(timing);
			setBusy(false);
		};
		return worker;
	}, []);

	return (
		<div className="w-4/5 max-w-[1000px]">
			<DaySelector day={day} setDay={setDay} />
			<PuzzleInput inputData={inputData} setInputData={setInputData} />
			<div className="m-2 w-full flex items-center">
				<Button
					variant="contained"
					onClick={() => {
						doComputation({
							day,
							inputData,
							setOutputData,
							setTimingData,
							setBusy,
							worker,
						});
					}}
					disabled={worker === null || inputData.length === 0 || busy}
				>
					Run solution
				</Button>
				{busy ? <CircularProgress size={24} className="mx-4" /> : null}
			</div>
			<PuzzleOutput output={outputData} timingData={timingData} />
		</div>
	);
}
