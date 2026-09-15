export async function constructRequest(
  kind: string,
  x: number,
  y: number,
  width: number,
  height: number,
): Promise<Response> {
  const url = "/api/ask_for_construct";
  const response = await fetch(url, {
    method: "POST",
    body: JSON.stringify({
      kind,
      pos: { x, y },
      size: { width, height },
    }),
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    },
  });

  return response;
}

export async function voteForConstruct(
  id: string,
  vote: number,
): Promise<Response> {
  const url = "/api/vote_for_construct";

  let vote_kind;
  switch (vote) {
    case 1:
      vote_kind = "UpVote";
      break;
    case -1:
      vote_kind = "DownVote";
      break;
    default:
      throw new Error(`Invalid vote value: ${vote}`);
  }

  const response = await fetch(url, {
    method: "POST",
    body: JSON.stringify({
      id,
      vote: vote_kind,
    }),
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    },
  });

  return response;
}
