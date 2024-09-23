import grpc
import task_pb2
import task_pb2_grpc

def run():
    with grpc.insecure_channel('localhost:50051') as channel:
        stub = task_pb2_grpc.TaskServiceStub(channel)

        # Create a task
        create_response = stub.CreateTask(task_pb2.CreateTaskRequest(title="Sample Task", description="This is a sample task"))
        print("Created Task:", create_response.task)

        # List tasks
        list_response = stub.ListTasks(task_pb2.Empty())
        print("Tasks:")
        for task in list_response.tasks:
            print(f"- ID: {task.id}, Title: {task.title}, Status: {task.status}")

if __name__ == '__main__':
    run()

