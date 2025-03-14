document.addEventListener("DOMContentLoaded", () => {
    const forms = document.querySelectorAll(".auth-form");

    function showForm(formId: string) {
        forms.forEach(form => {
            form.classList.add("hidden");
        });

        const targetForm = document.getElementById(formId);
        if (targetForm) {
            targetForm.classList.remove("hidden");
        }
    }

    document.body.addEventListener("click", (e) => {
        const target = e.target as HTMLElement;
        if (target.matches(".toggle-login")) {
            e.preventDefault();
            showForm("login-form");
        } else if (target.matches(".toggle-signup")) {
            e.preventDefault();
            showForm("signup-form");
        } else if (target.matches(".toggle-forgot")) {
            e.preventDefault();
            showForm("forgot-form");
        }
    });
});
