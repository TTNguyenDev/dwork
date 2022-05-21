# dWork
   
 ## Workflow: 
 - Job owner create a job 
 - Developers or workers have to register into dwork with a small bond.
 - Developers submit a proposal to one or more job at 1 time. Developers's hour estimation must be less than 20% different from job owner
 - Job owner select 1 proposal to process
 - Developers do their work and submit
 - Owner check the result and make a payroll
   
 ## Planning:
 - Developers will have level base on their previous work and total money they received
 - Handle exception case like: workers not do their work, owner not do payroll
 - Handle bonus, tip, maintainance, fix bug, etc
 - Charge services fee

### new
```sh
near call $ID new '{}' --accountId $ID
```
### new_job by requester
```sh
//Register as a requester
near call $ID register '{"requester": true}' --accountId job_creator.testnet --amount 0.5

// Create new job: job_creator.testnet
near call $ID new_task '{"title": "Retweet LNC post", "description": "Please Retweet this https://twitter.com/LearnNear/status/1491130118055796737. Your account need at least 5000 real followers", "price": "1000000000000000000", "max_participants": 2, "duration": "99999999999999999"}' --accountId job_creator.testnet --depositYocto 2000000000000000000
```
### register_as_a_worker
```sh
near call $ID register '{"requester": false}' --accountId job_worker.testnet --amount 0.5
```

### submit_work
//attach 0.1 for each submission
//when task close we will refund 
```sh
near call $ID submit_work '{"task_id": "'$TASK_ID'", "proof": "https://github.com/vunguyendev/dupwork"}' --accountId job_worker.testnet 
```

### approve_work 
```sh
near call $ID approve_work '{"task_id": "'$TASK_ID'", "worker_id": "job_worker.testnet"}' --accountId job_creator.testnet 
```

### reject_work 
```sh
near call $ID reject_work '{"task_id": "'$TASK_ID'", "worker_id": "job_worker.testnet"}' --accountId job_creator.testnet
```

### mark_task_as_completed
```sh
near call $ID mark_task_as_completed '{"task_id": "'$TASK_ID'"}' --accountId job_creator.testnet
```

## Views 
```sh 
// get available_tasks
near view $ID available_tasks '{"from_index": 0, "limit": 10}'

Response example
View call: dev-1645542863744-61299495451094.available_tasks({"from_index": 0, "limit": 10})
[
  [
    'job_creator.testnet_83453377',
    {
      owner: 'job_creator.testnet',
      title: 'Retweet LNC post',
      description: 'Please Retweet this https://twitter.com/LearnNear/status/1491130118055796737. Your account need at least 5000 real followers',
      max_participants: 2,
      price: '1000000000000000000',
      proposals: [],
      available_until: '1645543006936448101'
    }
  ],
  [
    'job_creator.testnet_83453523',
    {
      owner: 'job_creator.testnet',
      title: 'Retweet LNC post',
      description: 'Please Retweet this https://twitter.com/LearnNear/status/1491130118055796737. Your account need at least 5000 real followers',
      max_participants: 2,
      price: '1000000000000000000',
      proposals: [],
      available_until: '1645543168411808341'
    }
  ],
  [
    'job_creator.testnet_83453651',
    {
      owner: 'job_creator.testnet',
      title: 'Retweet LNC post',
      description: 'Please Retweet this https://twitter.com/LearnNear/status/1491130118055796737. Your account need at least 5000 real followers',
      max_participants: 2,
      price: '1000000000000000000',
      proposals: [
        {
          account_id: 'job_worker.testnet',
          proof_of_work: 'https://github.com/vunguyendev/dupwork',
          is_approved: false
        }
      ],
      available_until: '1745543317976864393'
    }
  ],
  [
    'job_creator.testnet_83454176',
    {
      owner: 'job_creator.testnet',
      title: 'Retweet LNC post',
      description: 'Please Retweet this https://twitter.com/LearnNear/status/1491130118055796737. Your account need at least 5000 real followers',
      max_participants: 2,
      price: '1000000000000000000',
      proposals: [],
      available_until: '1745543874924929128'
    }
  ]
]

//get user info
near view $ID user_info '{"account_id": "job_creator.testnet"}'

Response example
{
  account_id: 'job_creator.testnet',
  user_type: { type: 'Requester', total_stake: '0', current_requests: 0 },
  completed_jobs: []
}
```
