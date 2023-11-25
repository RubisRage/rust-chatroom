struct UserError;

enum AppError {
    UserError(UserError),
}
