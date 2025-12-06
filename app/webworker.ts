import init, { initThreadPool, run_day } from "@/aoc2025_wasm/pkg/aoc2025_wasm";

function warmup() {
	for (let day = 1; day <= 12; day++) {
		try {
			run_day(day, "");
		} catch (err) {
			console.log(`failed·to·warmup·day·${day}:·${err}`);
		}
	}
}

init().then(() => {
	initThreadPool(navigator.hardwareConcurrency).then(() => {
		warmup();
		postMessage(["", null]);
	});
});

// biome-ignore lint/suspicious/noGlobalAssign: see https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers
onmessage = (msg) => {
	const [day, input, bench] = msg.data;
	if (bench) {
		const startTime = performance.now();
		let endTime = startTime;
		let iterations = 0;
		while (endTime < startTime + 5000) {
			iterations++;
			run_day(day, input)
			endTime = performance.now();
		}
		postMessage([`${iterations} iterations in ${(endTime - startTime).toFixed(3)} ms, ${((endTime - startTime) / iterations).toFixed(3)} ms/iter`, null])
	} else {
		let workerResult = "";
		const startTime = performance.now();
		try {
			workerResult = run_day(day, input);
		} catch {
			workerResult = "Error: wasm panic";
		}
		const endTime = performance.now();
		postMessage([workerResult, endTime - startTime]);
	}
};
