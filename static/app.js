const questionInput = document.getElementById('question');
const sendBtn = document.getElementById('send-btn');
const messagesDiv = document.getElementById('messages');

function appendMessage(text, sender) {
    const div = document.createElement('div');
    div.className = sender;
    if (sender === 'bot') {
        div.innerHTML = marked.parse(text);
    } else {
        div.textContent = text;
    }
    messagesDiv.appendChild(div);
    messagesDiv.parentElement.scrollTop = messagesDiv.parentElement.scrollHeight;
}

async function sendQuestion() {
    const q = questionInput.value.trim();
    if (!q) return;
    appendMessage(q, 'user');
    questionInput.value = '';
    questionInput.disabled = true;
    sendBtn.disabled = true;

    try {
        const res = await fetch('/ask', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ question: q }),
        });
        const data = await res.json();
        appendMessage(data.answer, 'bot');
    } catch (err) {
        appendMessage('Error: ' + err.message, 'bot');
    }
    questionInput.disabled = false;
    sendBtn.disabled = false;
    questionInput.focus();
}

sendBtn.addEventListener('click', sendQuestion);
questionInput.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') sendQuestion();
});
