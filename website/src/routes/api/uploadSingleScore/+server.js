export const POST = ({request}) => {
    const scoreData = request.json();
    console.log(scoreData);

    return new Response("respnose :Ds")
}
