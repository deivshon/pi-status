import './Ram.css'
import { formatBytes } from "../../utils"

import RamStat from './RamStat'

export default function Ram({ramUsage}) {
    return <div class="stats-container flex-column align-items-center w-100">
            <div>
                {ramUsage.total ? formatBytes(ramUsage.total) : 0}
            </div>
            <div></div>
            <div className="w-100 d-flex flex-column align-items-center">
                <RamStat
                    total={ramUsage.total || 0}
                    stat={ramUsage.used ? ramUsage.used : 0}
                    label="Used"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <RamStat
                    total={ramUsage.total || 0}
                    stat={ramUsage.available ? ramUsage.available : 0}
                    label="Available"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <RamStat
                    total={ramUsage.total || 0}
                    stat={ramUsage.free ? ramUsage.free : 0}
                    label="Free"
                />
            </div>
            <div className="w-100 d-flex flex-column align-items-center">
                <RamStat
                    total={ramUsage.total || 0}
                    stat={ramUsage.cached ? ramUsage.cached : 0}
                    label="Cached"
                />
            </div>
    </div>
}
