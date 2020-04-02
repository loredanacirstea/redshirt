# Redshirt


## OS boot


```sequence
participant Kernel
participant SystemBuilder
participant System
participant ipc_Core
participant ipc_Process
participant extr_ProcessesCollectionExtrinsics
participant proc_ProcessesCollection
participant ipc_CoreProcess
participant vm_ProcessStateMachine

Kernel -> SystemBuilder: new()


SystemBuilder -> ipc_Core: new()
ipc_Core -> SystemBuilder: core builder
Note over ipc_Core: interface_interface_pid \n load_source_virtual_pid
SystemBuilder -> ipc_Core: reserve pids

SystemBuilder -> Kernel: system builder

Kernel -> SystemBuilder: add programs
Kernel -> SystemBuilder: build()
SystemBuilder -> ipc_Core: build()
ipc_Core -> SystemBuilder: core

Note over ipc_Core: system handled \n interfaces
SystemBuilder -> ipc_Core: set_interface_handler(INTERFACE, pid)

SystemBuilder -> Kernel: System

Note over Kernel: loop over \n cli programs
Kernel -> System: execute cli program
System -> ipc_Core: execute cli program
ipc_Core -> ipc_Process: new()
ipc_Process -> ipc_Core: proc_metadata
Note over ipc_Core: self.processes
ipc_Core -> extr_ProcessesCollectionExtrinsics: execute \n (module, proc_metadata)

Note over extr_ProcessesCollectionExtrinsics: self.inner
extr_ProcessesCollectionExtrinsics -> proc_ProcessesCollection: execute \n (module, proc_user_data,\n main_thread_user_data)


proc_ProcessesCollection -> vm_ProcessStateMachine: new() \n module, main_thread_data
vm_ProcessStateMachine -> proc_ProcessesCollection: state_machine
proc_ProcessesCollection -> proc_Process: new() \n state_machine, proc_user_data
proc_Process -> proc_ProcessesCollection: process
proc_ProcessesCollection -> proc_ProcessesCollection: self.processes.insert \n pid, process
proc_ProcessesCollection -> extr_ProcessesCollectionExtrinsics: ProcessesCollectionProc \n process, tid_pool

Note over ipc_Core: ProcessesCollectionProc pid
extr_ProcessesCollectionExtrinsics -> ipc_Core: ProcessesCollectionExtrinsicsProc \n self, pid, user_data_proc

ipc_Core -> System: ipc_CoreProcess { process }
System -> Kernel: process pid

Note over Kernel: loop
Kernel -> System: run()
Note over System: loop
Note over System: OS run

System -> Kernel: sys_SystemRunOutcome \n pid
Note over Kernel: exit if pids \n have finished or error

```



## OS Run Loop once


```sequence
participant Kernel
participant System
participant ipc_Core

Kernel -> System: run()

System -> System: load programs
Note over System: loop
System -> ipc_Core: emit_interface_message_answer \n load_source_virtual_pid, \n loader interface, LoaderMessage
ipc_Core -> ipc_Core: build notification \n syscalls
ipc_Core -> ipc_Core: try_resume_notification_wait

ipc_Core -> System: message_id
System -> System: add to loading_programs

Note over System: run
System -> System: run_once()


Note over System: RunOnceOutcome \n event Ready/Loop/continue
Note over System: native_programs
System -> System: next_event()

System -> ipc_Core: emit_interface_answer* \n answer_message \n cancel_message

```



## System run_once


```sequence
participant System
participant ipc_Core
participant extr_ProcessesCollectionExtrinsics
participant proc_ProcessesCollection
participant vm_Thread
participant native_NativeProgramsCollection

System -> System: run_once()
System -> ipc_Core: run()

Note over ipc_Core: if pending_events \n return self.pending_events.pop()
Note over ipc_Core: else \n self.processes.run()
ipc_Core -> extr_ProcessesCollectionExtrinsics: run()

Note over extr_ProcessesCollectionExtrinsics: self.inner
extr_ProcessesCollectionExtrinsics -> proc_ProcessesCollection: run()

Note over proc_ProcessesCollection: find thread \n ready to run \n self.processes
Note over proc_ProcessesCollection: process.state_machine.thread \n thread.user_data().value_back.take()
proc_ProcessesCollection -> vm_Thread: run \n (value_back: wasmi::RV)

Note over vm_Thread: self.vm.threads[self.index] \n vm_ThreadState
vm_Thread -> vm_ThreadState: execution.resume_execution() | \n execution.start_execution()
Note over vm_ThreadState: execution: \n wasmi::FuncInvocation
vm_ThreadState -> vm_Thread: return_value

Note over vm_Thread: return: \n ThreadFinished \n Interrupted \n Errored
vm_Thread -> proc_ProcessesCollection: ExecOutcome

Note over proc_ProcessesCollection: return: \n ProcessFinished \n ThreadFinished \n Idle \n Interrupted \n
proc_ProcessesCollection -> extr_ProcessesCollectionExtrinsics: proc_RunOneOutcome

Note over extr_ProcessesCollectionExtrinsics: return: \n ProcessFinished \n ThreadFinished \n ThreadWaitNotification \n ThreadEmitAnswer \n ThreadEmitMessageError \n ThreadCancelMessage \n Idle

extr_ProcessesCollectionExtrinsics -> ipc_Core: RunOneOutcome


Note over ipc_Core: return: \n Idle \n ProgramFinished \n ThreadWaitUnavailableInterface \n  MessageResponse \n ReservedPidInterfaceMessage
ipc_Core -> System: CoreRunOutcome

System -> System: RunOnceOutcome


```

## System run_once Core outcomes


```sequence

participant System
participant ipc_Core
participant extr_ProcessesCollectionExtrinsics

extr_ProcessesCollectionExtrinsics -> ipc_Core: RunOneOutcome

Note over ipc_Core: if ProcessFinished \n unregister interfaces \n cancel messages \n syscall process destroyed
ipc_Core -> System: CoreRunOutcome::ProgramFinished


Note over ipc_Core: if ThreadWaitNotification \n try_resume_notification_wait_thread
ipc_Core -> System: None


Note over ipc_Core: if ThreadEmitMessage \n syscall send message
ipc_Core -> System: None \ CoreRunOutcome::ReservedPidInterfaceMessage


Note over ipc_Core: if ThreadEmitAnswer / ThreadEmitMessageError
Note over ipc_Core: -- if emitter_pid && process \n syscall response_notification \n try_resume_notification_wait
ipc_Core -> System: None

Note over ipc_Core: -- if emitter_pid && !process
ipc_Core -> System: CoreRunOutcome::MessageResponse



Note over ipc_Core: if ThreadCancelMessage
ipc_Core -> System: None


Note over ipc_Core: if Idle
ipc_Core -> System: CoreRunOutcome::Idle
```

## System run_once outcomes



```sequence

Note over System: if MessageResponse \n load & execute wasm
System -> ipc_Core: execute(module)
ipc_Core -> System: None

Note over System: if ReservedPidInterfaceMessage \n && interface_interface

System -> ipc_Core: set_interface_handler
ipc_Core -> System: result
System -> System: InterfaceRegisterResponse \n (result, interface)
Note over System: if message_id
System -> ipc_Core: answer_message \n (message_id, response)
Note over System: if module_loader interface
System -> System: LoopAgainNow

Note over System: if ReservedPidInterfaceMessage \n self.native_programs
System -> native_NativeProgramsCollection: interface_message \n (interface, message_id, pid, message)

Note over native_NativeProgramsCollection: loop self.processes
native_NativeProgramsCollection -> Adapter_NativeProgramRef: deliver_interface_message \n (interface, message_id, \n emitter_pid, msg)

Note over Adapter_NativeProgramRef: self.inner
Adapter_NativeProgramRef -> NativeProgramRef: interface_message \n (interface, message_id, \n emitter_pid, message)

NativeProgramRef -> Adapter_NativeProgramRef: -
Adapter_NativeProgramRef -> native_NativeProgramsCollection: -
native_NativeProgramsCollection -> System: - / Err msg


```





```uml
http://192.168.1.140:4000/info.txt
```
