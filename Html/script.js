document.addEventListener("DOMContentLoaded", function() {
    const table = document.querySelector("table");
    const from_web = fetch('https://theisland.fly.dev/leaderboard');
    from_web.then(response => table.innerHTML = response.text());
});