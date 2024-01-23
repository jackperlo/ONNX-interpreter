import Group17

def main():
    Group17.onnx_make_inference("models/mnist-8.onnx", "mnist_data_0.pb", "mnist_output_0.pb",
                                ["Input3", "Parameter193"])


if __name__ == "__main__":
    main()
