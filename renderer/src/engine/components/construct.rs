/// Building component currently being voted.
#[derive(bevy::prelude::Component)]
#[require(bevy::prelude::Transform, bevy::prelude::Visibility)]
pub struct Construct {
    building: api::common::Building,
    status: api::common::ConstructStatus,
}

impl Construct {
    /// Create a new construct component
    pub fn new(construct: api::common::Construct) -> Self {
        Self {
            building: construct.building,
            status: construct.status,
        }
    }

    /// Get the title of the building for in game informations.
    pub fn title(&self) -> String {
        self.building.kind.to_string()
    }

    /// Get the description of the building for the in game informations.
    pub fn description(&self) -> String {
        fn get_remaining_voting_time(construct_time: chrono::DateTime<chrono::Utc>) -> Option<chrono::Duration> {
            let now = chrono::Utc::now();
            let voting_deadline = construct_time.checked_add_days(chrono::Days::new(1))?;
            Some(voting_deadline - now)
        }

        let remaining_time_text = match get_remaining_voting_time(self.building.created_at) {
            Some(remaining) => {
                if remaining >= chrono::Duration::zero() {
                    let rem_hours = remaining.num_hours();
                    let rem_minutes = (remaining.num_minutes() % 60) + 60 % 60;
                    let rem_seconds = (remaining.num_seconds() % 60) + 60 % 60;
                    format!("Votes end in {rem_hours}:{rem_minutes:0>2}:{rem_seconds:0>2}")
                } else {
                    format!("Vote ended!")
                }
            }
            None => format!("Asked the {}", self.building.created_at.format("%Y-%m-%d at %H:%M")),
        };

        format!(
            "A cute lil house.\nVotes: {} / {}\n{}",
            self.status.votes, self.status.cost, remaining_time_text
        )
    }

    /// Get the grid position of this construct
    pub fn grid_pos(&self) -> crate::engine::components::grid::GridPos {
        crate::engine::components::grid::GridPos {
            x: self.building.pos.x,
            y: self.building.pos.y,
        }
    }

    /// Get the construct id
    pub fn id(&self) -> uuid::Uuid {
        self.building.id
    }

    /// Get the building
    pub fn building(&self) -> api::common::Building {
        self.building.clone()
    }

    /// Update the construct status
    pub fn update_status(&mut self, new_status: api::common::ConstructStatus) {
        self.status = new_status;
    }
}

/// Marker component for a construct being selected by the user
#[derive(bevy::prelude::Component)]
#[require(bevy::prelude::Transform, bevy::prelude::Visibility)]
pub struct SelectedConstruct;
