import classNames from "classnames";

type ErrorBoxProps = {
    error: string;
};

export const ErrorBox = (props: ErrorBoxProps) => {
    const len = props.error.length;

    return (
        <div className="flex h-full items-center justify-center">
            <div
                className={classNames(
                    "no-scrollbar max-h-[50vh] max-w-[80vw] overflow-y-scroll break-words rounded-xl bg-red-400 px-4 py-2 md:w-[50vw]",
                    {
                        "text-xs md:text-xs": len >= 500,
                        "text-xs md:text-sm": len >= 300 && len < 500,
                        "text-sm md:text-base": len < 300,
                    },
                )}
            >
                <p className="text-center text-red-950">{props.error}</p>
            </div>
        </div>
    );
};
