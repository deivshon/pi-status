type ProgressBarProps = {
    percentage: number;
    className?: string;
    bgColor: string;
    fgColor: string;
};

export const ProgressBar = (props: ProgressBarProps) => {
    return (
        <div
            className={
                "w-full overflow-hidden rounded-md " + (props.className ?? "")
            }
            style={{ backgroundColor: props.bgColor }}
        >
            <div
                className={
                    "h-full rounded-none transition-all duration-500 ease-out "
                }
                style={{
                    width: `${props.percentage}%`,
                    backgroundColor: props.fgColor,
                }}
            ></div>
        </div>
    );
};
