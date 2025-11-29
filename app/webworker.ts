import init, { initThreadPool, run_day } from "@/aoc2025_wasm/pkg/aoc2025_wasm";

let init_done = false;

function warmup() {
    for (let day = 1; day <= 12; day++) {
        run_day(day, "");
    }
}

function post_answer(day: number, input: string): void {
    const startTime = performance.now();
    const workerResult = run_day(day, input);
    const endTime = performance.now();
    postMessage([workerResult, endTime - startTime]);
}

// biome-ignore lint/suspicious/noGlobalAssign: see https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers
onmessage = (msg) => {
    const [day, input] = msg.data;
    if (!init_done) {
        init().then(() => {
            initThreadPool(navigator.hardwareConcurrency).then(() => {
                warmup();
                init_done = true;
                post_answer(day, input);
            });
        });
    } else {
        post_answer(day, input);
    }
};
