
#[cfg(test)]
mod state_instance {

    use crate::environment::agent::Agent;
    use crate::environment::state::State;

    #[test]
    fn test_init_state() {

        /* 2d environment */ 
        let state  = State::new("testing-state", vec![3, 3]);
        let state_dim = state.dim();
        let expected_dims: Vec<usize> = vec![3, 3];
        let expected_vals: Vec<i32> = vec![0,0,0,0,0,0,0,0,0];

        assert_eq!(state.name(), "testing-state"); 
        assert_eq!(state_dim, &expected_dims); 
        assert_eq!(state.grid().values(), &expected_vals);
        assert_eq!(state.turn(), 0);  


        let state_3d  = State::new("testing-state-3d", vec![2, 3, 3]);
        let state_dim_3d = state_3d.dim();
        let expected_dims_3d: Vec<usize> = vec![2, 3, 3];
        let expected_vals_3d: Vec<i32> = vec![
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0
        ];

        assert_eq!(state_3d.name(), "testing-state-3d"); 
        assert_eq!(state_dim_3d, &expected_dims_3d); 
        assert_eq!(state_3d.grid().values(), &expected_vals_3d);
        assert_eq!(state_3d.turn(), 0);  

    }

    #[test]
    fn test_place() {

        let mut state  = State::new("testing-place-state", vec![3, 3]);
        let state_dim = state.dim();
        let expected_dims: Vec<usize> = vec![3, 3];
        let expected_vals: Vec<i32> = vec![0,0,0,0,0,0,0,0,0];

        assert_eq!(state_dim, &expected_dims); 
        assert_eq!(state.grid().values(), &expected_vals);

        state.place(2, vec![0, 0]); 
        state.place(2, vec![1, 1]); 
        state.place(2, vec![2, 2]);
        
        /* validate new vals */ 
        let new_vals: Vec<i32> = vec![2,0,0,0,2,0,0,0,2];
        assert_eq!(state.grid().values(), &new_vals);

        let mut state_3d  = State::new("testing-state-3d", vec![3, 3, 3]);
        let state_dim_3d = state_3d.dim();
        let expected_dims_3d: Vec<usize> = vec![3, 3, 3];
        let expected_vals_3d: Vec<i32> = vec![
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0
        ];

        assert_eq!(state_3d.name(), "testing-state-3d"); 
        assert_eq!(state_dim_3d, &expected_dims_3d); 
        assert_eq!(state_3d.grid().values(), &expected_vals_3d);
        assert_eq!(state_3d.turn(), 0); 
        
        state_3d.place(2, vec![0, 0, 0]); 
        state_3d.place(2, vec![1, 1, 1]); 
        state_3d.place(2, vec![2, 2, 2]);

        let expected_vals_3d: Vec<i32> = vec![
            2,0,0,0,0,0,0,0,0,
            0,0,0,0,2,0,0,0,0,
            0,0,0,0,0,0,0,0,2
        ];
        assert_eq!(state_3d.grid().values(), &expected_vals_3d);


    }


    #[test]
    fn test_resize_state() {

        let mut state  = State::new("testing-resize", vec![3, 3]);
        let state_dim = state.dim();
        let expected_dims: Vec<usize> = vec![3, 3];
        let expected_vals: Vec<i32> = vec![0,0,0,0,0,0,0,0,0];

        assert_eq!(state.name(), "testing-resize"); 
        assert_eq!(state_dim, &expected_dims); 
        assert_eq!(state.grid().values(), &expected_vals);
        assert_eq!(state.turn(), 0); 
        
        state.resize(vec![3,3,3]);

        let state_dim_3d = state.dim();
        let expected_dims_3d: Vec<usize> = vec![3, 3, 3];
        let expected_vals_3d: Vec<i32> = vec![
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0
        ];


        assert_eq!(state.name(), "testing-resize"); 
        assert_eq!(state_dim_3d, &expected_dims_3d); 
        assert_eq!(state.grid().values(), &expected_vals_3d);
        assert_eq!(state.turn(), 0); 
 

    }

    #[test]
    fn test_add_agent() {

        let mut state  = State::new("testing-add-agent", vec![3, 3]);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");

        state.add_agent(agent1);
        state.add_agent(agent2); 

        let expected_names = vec!["agent-1", "agent-2"]; 
        let expected_ids = vec![1, 2];

        let mut counter = 0; 
        for item in state.agents() {
            assert_eq!(item.id(), expected_ids[counter]); 
            assert_eq!(item.label(), expected_names[counter]);
            assert_eq!(item.score(), 0.0); 
            counter += 1;  
        }

    }

    #[test]
    fn test_actions() {

        let mut state  = State::new("testing-state-actions", vec![3, 3]);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");

        state.add_agent(agent1.clone());
        state.add_agent(agent2.clone()); 

        let expected_actions = vec![
            [0,0], [0,1], [0,2], 
            [1,0], [1,1], [1,2], 
            [2,0], [2,1], [2,2]
        ];

        let actions = state.actions();
        assert_eq!(actions, expected_actions);

        state.place(agent1.id(), vec![0, 0]);
        state.place(agent2.id(), vec![2, 2]);

        let expected_actions_1 = vec![
            [0,1], [0,2], 
            [1,0], [1,1], [1,2], 
            [2,0], [2,1]
        ];

        let actions = state.actions();
        assert_eq!(actions, expected_actions_1);
    }


    #[test]
    fn test_save_load_state() {

        let mut state  = State::new("testing-state-save-load", vec![3, 3]);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");

        state.add_agent(agent1.clone());
        state.add_agent(agent2.clone());

        state.place(agent1.id(), vec![0, 0]);
        state.place(agent2.id(), vec![2, 2]);

        let _ = state.save("data/tictactoe");
        let loaded: State = State::load("data/tictactoe").unwrap();

        let state_dim = loaded.dim();
        let expected_dims: Vec<usize> = vec![3, 3];
        let expected_vals: Vec<i32> = vec![1,0,0,0,0,0,0,0,2];

        assert_eq!(loaded.name(), "testing-state-save-load"); 
        assert_eq!(state_dim, &expected_dims); 
        assert_eq!(loaded.grid().values(), &expected_vals);
        assert_eq!(loaded.turn(), 0); 

        
    }


    #[test]
    fn test_agent_cycle() {

        let mut state  = State::new("testing-state-save-load", vec![3, 3]);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");
        let agent3 = Agent::new(3, "agent-3");

        state.add_agent(agent1.clone());
        state.add_agent(agent2.clone());
        state.add_agent(agent3.clone());

    }


}


#[cfg(test)]
mod state_agent {

    use crate::environment::agent::Agent;

    /* tests for the board instance */

    #[test]
    fn test_init_agent() {

        let mut agent = Agent::new(1, "agent-1");
        assert_eq!(agent.id(), 1);
        assert_eq!(agent.label(), "agent-1"); 
        assert_eq!(agent.active(), false); 
        assert_eq!(agent.score(), 0.0); 

        /* make agent active */ 
        agent.status(true); 
        assert_eq!(agent.active(), true);

    }

    #[test]
    fn test_agent_score() {

        let mut agent = Agent::new(1, "agent-1");
        assert_eq!(agent.id(), 1);
        assert_eq!(agent.label(), "agent-1"); 
        assert_eq!(agent.active(), false); 
        assert_eq!(agent.score(), 0.0); 

        /* make agent active */ 
        agent.status(true); 
        assert_eq!(agent.active(), true);

        agent.set_score(20.0); 
        assert_eq!(agent.score(), 20.0); 

        agent.add_score(10.0); 
        assert_eq!(agent.score(), 30.0); 

    }

    #[test]
    fn test_save_load_agent() {

        let mut agent = Agent::new(1, "agent-1");
        assert_eq!(agent.id(), 1);
        assert_eq!(agent.label(), "agent-1"); 
        assert_eq!(agent.active(), false); 
        assert_eq!(agent.score(), 0.0); 

        /* make agent active */ 
        agent.status(true); 
        assert_eq!(agent.active(), true);

        agent.set_score(20.0); 
        assert_eq!(agent.score(), 20.0); 

        /* test save and load functions */ 
        let _ = agent.save("data/agent1");
        let mut loaded: Agent = Agent::load("data/agent1").unwrap();

        loaded.add_score(10.0); 
        assert_eq!(loaded.score(), 30.0);
        assert_eq!(loaded.id(), 1);
        assert_eq!(agent.label(), "agent-1"); 
        assert_eq!(agent.active(), true); 

    }

}