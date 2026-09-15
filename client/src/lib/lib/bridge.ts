import { voteForConstruct } from "./api";
import { pushNotification } from "./notifications";

/** Vote for a given construct */
export function vote_for_construct(construct_id: string, vote: number) {
  voteForConstructImpl(construct_id, vote);
}

async function voteForConstructImpl(construct_id: string, vote: number) {
  const response = await voteForConstruct(construct_id, vote);

  const action_name = vote == 1 ? "upvoted" : "downvoted";

  if (response.ok) {
    pushNotification(`You ${action_name} a building!`);
  } else {
    const error = await response.text();
    pushNotification(error, "red");
  }
}
