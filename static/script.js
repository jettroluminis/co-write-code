document.addEventListener('DOMContentLoaded', () => {
    const guessInput = document.getElementById('guess');
    const submitButton = document.getElementById('submit');
    const outputDiv = document.getElementById('output');

    submitButton.addEventListener('click', async () => {
        const guess = guessInput.value;
        if (guess) {
            const response = await fetch('/guess', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded',
                },
                body: `guess=${guess}`,
            });

            const result = await response.text();
            outputDiv.innerText = result;
        }
    });
});
