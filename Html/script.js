document.addEventListener("DOMContentLoaded", function () {
    const url = 'https://theisland.fly.dev/leaderboard';
    const imageUrl = 'https://theisland.fly.dev/topimages';
    const leaderboardTable = document.querySelector("tbody");
    const from_web = fetch(url);
    const webImage = fetch(imageUrl);

    var rank = 1;
    from_web.then(response => {
        response.json().then(data => {
            for (let leaderboardEntry of data) {
                let row = document.createElement("tr");

                let rankElement = document.createElement("td");
                rankElement.innerText = rank;
                row.appendChild(rankElement);
                if (rank <= 3) {
                    rankElement.style.color = "LightGreen";
                } else {
                    rankElement.style.color = "black";
                }

                let nameElement = document.createElement("td");
                nameElement.innerText = leaderboardEntry.person;
                row.appendChild(nameElement);
                if (rank <= 3) {
                    nameElement.style.color = "LightGreen";
                } else {
                    nameElement.style.color = "black";
                }

                let scoreElement = document.createElement("td");
                scoreElement.innerText = leaderboardEntry.score;
                row.appendChild(scoreElement);
                if (rank <= 3) {
                    scoreElement.style.color = "LightGreen";
                } else {
                    scoreElement.style.color = "black";
                }

                leaderboardTable.appendChild(row);

                rank = rank + 1;
            }
        });
    });

    webImage.then(response => {
        response.json().then(data => {
            const imageContainer = document.querySelector("div#image_container");

            const imageColumn = document.createElement("div");
            imageColumn.classList.add("image-column");

            const nameColumn = document.createElement("div");
            nameColumn.classList.add("image-column");

            for (let imageEntry of data) {
                let frame = document.createElement("div");
                frame.classList.add("image-frame");

                let imageElement = document.createElement("img");
                imageElement.src = imageEntry.image;

                let headingElement = document.createElement("div");
                headingElement.classList.add("name-frame");

                let headingName = document.createElement("h1");
                headingName.innerText = imageEntry.person;

                headingElement.appendChild(headingName);
                nameColumn.appendChild(headingElement);


                frame.appendChild(imageElement);
                imageColumn.appendChild(frame);

            }




            imageContainer.appendChild(imageColumn);
            imageContainer.appendChild(nameColumn);



            for (let imageEntry of data) {
                let imageRow = document.createElement("tr");

                let personElement = document.createElement("td");
                personElement.innerText = imageEntry.person;
                imageRow.appendChild(personElement);

                let image_scoreElement = document.createElement("td");
                image_scoreElement.innerText = imageEntry.image_score;
                imageRow.appendChild(image_scoreElement);

                let imageTableElement = document.createElement("td");
                let actualImageElement = document.createElement("image");
                actualImageElement.src = imageEntry.image;
                imageTableElement.appendChild(actualImageElement);

                imageRow.appendChild(imageTableElement);
            }
        });
    });
});