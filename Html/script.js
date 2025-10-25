document.addEventListener("DOMContentLoaded", function() {
    const url = 'https://theisland.fly.dev/leaderboard';
    const table = document.querySelector("tbody");
    //fetches resources from the web

    const from_web = fetch(url);
    //updates the leaderboard, response as a string
    from_web.then(response => {
        response.json().then(data => {
            for (let leaderboardEntry of data) {
                let row = document.createElement("tr");

                let nameElement = document.createElement("td");
                nameElement.innerText = leaderboardEntry.person;
                row.appendChild(nameElement);


                table.appendChild(row);
            }
        });
    });
});