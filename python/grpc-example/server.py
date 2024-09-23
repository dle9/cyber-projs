import grpc
from concurrent import futures
import task_pb2
import task_pb2_grpc

class TaskService(task_pb2_grpc.TaskServiceServicer):
    def __init__(self):
        self.tasks = {}

    def CreateTask(self, request, context):
        task_id = str(len(self.tasks) + 1)
        task = task_pb2.Task(id=task_id, title=request.title, description=request.description, status="pending")
        self.tasks[task_id] = task
        return task_pb2.TaskResponse(task=task)

    def GetTask(self, request, context):
        task = self.tasks.get(request.id)
        if task:
            return task_pb2.TaskResponse(task=task)
        context.set_code(grpc.StatusCode.NOT_FOUND)
        context.set_details("Task not found")
        return task_pb2.TaskResponse()

    def UpdateTask(self, request, context):
        task = self.tasks.get(request.id)
        if task:
            task.title = request.title
            task.description = request.description
            task.status = request.status
            return task_pb2.TaskResponse(task=task)
        context.set_code(grpc.StatusCode.NOT_FOUND)
        context.set_details("Task not found")
        return task_pb2.TaskResponse()

    def DeleteTask(self, request, context):
        if request.id in self.tasks:
            del self.tasks[request.id]
            return task_pb2.Empty()
        context.set_code(grpc.StatusCode.NOT_FOUND)
        context.set_details("Task not found")
        return task_pb2.Empty()

    def ListTasks(self, request, context):
        return task_pb2.ListTasksResponse(tasks=list(self.tasks.values()))

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    task_pb2_grpc.add_TaskServiceServicer_to_server(TaskService(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    print("Server is running on port 50051...")
    server.wait_for_termination()

if __name__ == '__main__':
    serve()

