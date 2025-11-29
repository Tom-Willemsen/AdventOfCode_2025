"use client";

import { useMemo, useState } from "react";
import DaySelector from "./day_selector";
import PuzzleInput from "./puzzle_input";
import PuzzleOutput from "./puzzle_output";
import RunButton from "./run_button";

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

	// Initially busy while web worker sets up wasm. Set back to false by an
	// initial message from worker.
	const [busy, setBusy] = useState(true);

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
			<RunButton
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
				busy={busy}
				disabled={inputData.length === 0}
			/>
			<PuzzleOutput output={outputData} timingData={timingData} />
		</div>
	);
}
