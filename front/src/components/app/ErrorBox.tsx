type ErrorBoxProps = {
    error: string;
};

export const ErrorBox = (props: ErrorBoxProps) => {
    return (
        <div
            className="h-100 d-flex justify-content-center align-items-center container"
            id="error-box"
        >
            <div className="col-md-6">
                <p className="alert alert-danger text-center" role="alert">
                    {props.error}
                </p>
            </div>
        </div>
    );
};
