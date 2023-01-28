import './Cpu.css'

import { useState } from 'react'
import { useEffect } from "react";

export default function Cpu({coreUsage}) {
    const [usage, setUsage] = useState(0)

    useEffect(() => {
        let total = coreUsage.user +
                    coreUsage.system +
                    coreUsage.irq +
                    coreUsage.softirq +
                    coreUsage.idle +
                    coreUsage.iowait;

        let idle = coreUsage.idle + coreUsage.iowait

        setUsage(((total - idle) / total) * 100)
    })

    return <div class="progress w-100">
        <div class="progress-bar cpu-bar" role="progressbar" style={{width: `${usage}%`}}></div>
    </div>
}