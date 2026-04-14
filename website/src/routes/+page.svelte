<script>
    const { data } = $props();
</script>

<table>
    <thead>
        <tr>
            <th>Level</th>
            <th>Sum</th>
            <th>Winner</th>
            <th>Cost</th>
            <th>Cycles</th>
            <th>Area</th>
        </tr>
    </thead>
    <tbody>
        {#each data.chapterList as chapter}
            <tr><th>{chapter.title}</th></tr>
            {#each chapter.levels as score}
                <tr class={score.username}>
                    <td>{score.level}</td>
                    <td>{score.sum}</td>
                    <td>{score.username}</td>
                    <td>{score.cost}</td>
                    <td>{score.cycles}</td>
                    <td>{score.area}</td>
                </tr>
            {/each}
        {/each}
    </tbody>
</table>

<style>
    table {
        border-collapse: collapse;
        border: 2px solid black;
        font-size: 1.2rem;
    }

    thead {
        border-bottom: 1px solid gray;
    }

    td,
    th {
        border: 1px solid gray;
    }

    td,
    th {
        text-align: center;
        padding: 0.25em 1.5em;
    }

    tr.koki {
        background-color: #b7e1cd;
    }

    tr.noki {
        background-color: #a4c2f4;
    }

    tr.pooki {
        background-color: #ffd966;
    }
</style>
