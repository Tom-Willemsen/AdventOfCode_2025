'use client'

import init, { run_2025_01 } from "@/aoc2025_wasm/pkg/aoc2025_wasm";
import React, { useState, useEffect } from 'react';

export default function Home() {
  const [data, setData] = useState("");

  init().then(() => setData(run_2025_01("h")))

  return (
    <h4>{data}</h4>
  );
}
