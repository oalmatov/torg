document.addEventListener("DOMContentLoaded", () => {
    const forms = document.querySelectorAll(".auth-form");

    function showForm(formId) {
        forms.forEach(form => {
            form.classList.add("hidden"); // Hide all forms
        });
        document.getElementById(formId).classList.remove("hidden"); // Show selected form
    }

    // Event delegation: Attach event listeners to the whole document
    document.body.addEventListener("click", (event) => {
        if (event.target.matches(".toggle-login")) {
            event.preventDefault();
            showForm("login-form");
        } else if (event.target.matches(".toggle-signup")) {
            event.preventDefault();
            showForm("signup-form");
        } else if (event.target.matches(".toggle-forgot")) {
            event.preventDefault();
            showForm("forgot-form");
        }
    });
});
