const form = document.getElementById('shortenForm');
const resultDiv = document.getElementById('result');
const errorDiv = document.getElementById('error');
const shortUrlInput = document.getElementById('shortUrl');
const viewStatsLink = document.getElementById('viewStats');

form.addEventListener('submit', async (e) => {
    e.preventDefault();

    resultDiv.classList.add("hidden");
    errorDiv.classList.add("hidden");

    const url = document.getElementById("url").value;
    const customCode = document.getElementById("customCode").value;
    const expiresAt = document.getElementById("expiresAt").value;

    const payload = {
        url: url,
        custom_code: customCode || null,
        expires_at: expiresAt ? new Date(expiresAt).toISOString() : null
    };

    try {
        const response = await fetch("/api/shorten", {
            method: "POST",
            headers: {
                'Content-Type': "application/json",
            },
            body: JSON.stringify(payload)
        });

        const data = await response.json();

        if (!response.ok) {
            throw new Error(data.error || "Failed to shorten URL");
        }

        shortUrlInput.value = data.short_url;
        viewStatsLink.href = `/api/urls/${data.short_code}`;
        resultDiv.classList.remove("hidden");

        form.reset();
    } catch (error) {
        errorDiv.textContent = error.message;
        errorDiv.classList.remove("hidden");
    }
});

function copyUrl() {
    shortUrlInput.select();
    document.execCommand('copy');

    const button = event.target;
    const originalText = button.textContent;
    button.textContent = 'Copied!';

    setTimeout(() => {
        button.textContent = originalText;
    }, 2000)
}