from queue import Queue

import threading
import numpy as np

import serial

imu_queue = Queue()
is_thread_exit = False


class IMU:
    def __init__(self):
        self.ser = serial.Serial('/dev/tty.usbmodem1141301', 115200, timeout=1)
        self.ser.flush()
        self.quaternion = np.array([0, 0, 0, 0])
        self.accel = np.array([0, 0, 0])
        self.gyro = np.array([0, 0, 0])
        self.mag = np.array([0, 0, 0])

    def read(self):
        data = self.ser.readline().decode('utf-8').split(',')
        try:
            self.quaternion = np.array(
                [float(data[0]), float(data[1]), float(data[2]), float(data[3])])
            self.accel = np.array([float(data[4]), float(data[5]), float(data[6])])
            self.gyro = np.array([float(data[7]), float(data[8]), float(data[9])])
            self.mag = np.array([float(data[10]), float(data[11]), float(data[12])])
        except Exception as e:
            pass


class KalmanFilter:
    def __init__(self):
        self.dt = 0.01

        self.acc = np.zeros((3, 1))
        self.gyro = np.zeros((3, 1))

        # 状態ベクトル
        self.x = np.zeros((3, 1))

        # 制御入力
        self.u_k = np.zeros((3, 1))

        # 観測値
        self.z_k = np.zeros((3, 1))

        # 観測誤差
        self.y_k = np.zeros((3, 1))

        # 共分散行列
        self.S = np.zeros((3, 3))

        # カルマンゲイン
        self.K = np.zeros((3, 3))

        # 誤差共分散行列
        self.P = np.zeros((3, 3))

        # ヤコビアン行列
        self.F = np.zeros((3, 3))

        self.H = np.eye(3, 3)

        # 観測ノイズ共分散行列
        dt_pow2 = self.dt ** 2
        self.R = np.diag([dt_pow2, dt_pow2, dt_pow2])

        self.Q = np.diag([1.74E-2 * self.dt ** 2,
                          1.74E-2 * self.dt ** 2,
                          1.74E-2 * self.dt ** 2])

    def update_K(self):
        """
        カルマンゲインを計算する.
        :return:
        """
        self.K = self.P @ self.H.T @ np.linalg.inv(self.S)
        return self.K

    def calc_z(self):
        """
        加速度から指定を推定値を計算する.
        :return:
        """
        a_x, a_y, a_z = self.acc[0, 0], self.acc[1, 0], self.acc[2, 0]
        self.z_k = np.array([
            [np.arctan2(a_y, a_z)],
            [np.arctan2(-a_x, np.sqrt(a_y ** 2 + a_z ** 2))],
            [0]
        ])
        return self.z_k

    def update_y_res(self):
        """
        観測誤差を計算する.
        :return:
        """
        self.y_k = self.z_k - self.x
        return self.y_k

    def calc_u(self):
        """
        制御入力を計算する.
        ジャイロの値 $w_k$ にサンプリング間隔 $dt$ をかける.
        :return:
        """
        self.u_k = self.gyro * self.dt
        return self.u_k

    def calc_f(self):
        """
        状態方程式を解く.
        :return:
        """
        u_x, u_y, u_z = self.u_k[0, 0], self.u_k[1, 0], self.u_k[2, 0]
        c_x, s_x = np.cos(u_x), np.sin(u_x)
        c_y, s_y = np.cos(u_y), np.sin(u_y)
        c_z, s_z = np.cos(u_z), np.sin(u_z)

        self.x = np.array([
            [1, (s_x * s_y / c_y), (c_x * s_y / c_y)],
            [0, c_x, -s_x],
            [0, (s_x / c_y), (c_x / c_y)]
        ]) @ self.u_k
        return self.x

    def calc_F(self):
        """
        ヤコビアン行列を計算する.
        :return:
        """
        u_x, u_y, u_z = self.u_k[0, 0], self.u_k[1, 0], self.u_k[2, 0]
        c_x, s_x = np.cos(u_x), np.sin(u_x)
        c_y, s_y = np.cos(u_y), np.sin(u_y)
        c_z, s_z = np.cos(u_z), np.sin(u_z)
        self.F = np.array([
            [
                1 + u_y * c_x * s_y / c_y - u_z * s_x * s_y / c_y,
                u_y * s_y / c_y ** 2 + u_z * c_x / c_y ** 2,
                0
            ],
            [
                - u_y * s_x - u_z * c_x,
                1,
                0
            ],
            [
                u_y * c_x / c_y - u_z * s_x / c_y,
                u_y * s_x * s_y / c_y ** 2 + u_z * c_x * s_y / c_y ** 2,
                1
            ]
        ])
        return self.F

    def update_S(self):
        """
        観測誤差共分散行列を計算する.
        :return:
        """
        self.S = self.H @ self.P @ self.H.T + self.R
        return self.S

    def predict_x(self):
        """
        状態方程式を解く.
        :return:
        """
        self.x = self.calc_f()
        return self.x

    def predict_P(self):
        """
        誤差共分散行列を計算する.
        :return:
        """
        self.P = self.F @ self.P @ self.F.T + self.Q
        return self.P

    def update_P(self):
        """
        誤差共分散行列を計算する.
        :return:
        """
        I = np.eye(3, 3)
        self.P = (I - self.K @ self.H) @ self.P
        return self.P

    def update_x(self):
        """
        状態方程式を解く.
        :return:
        """
        self.x = self.x + self.K @ self.y_k
        return self.x

    def predict(self, imu: IMU):
        self.acc = imu.accel.reshape((3, 1))
        self.gyro = imu.gyro.reshape((3, 1))

        self.calc_u()
        self.calc_z()

        # predict
        self.calc_F()
        self.predict_x()
        self.predict_P()

        # update
        self.update_y_res()
        self.update_S()
        self.update_K()
        self.update_x()
        self.update_P()

        return self.x, self.P


def read_imu():
    imu = IMU()
    while is_thread_exit is False:
        imu.read()
        imu_queue.put(imu)


def main():
    # IMU データを取得するスレッドを立ち上げる
    thread = threading.Thread(target=read_imu)

    # スレッドを開始する
    thread.start()

    while True:
        # キーボードインタラプトで終了
        try:
            if not imu_queue.empty():
                continue
            imu = imu_queue.get()

            # カルマンフィルタを適用
            kalman_filter = KalmanFilter()
            x, P = kalman_filter.predict(imu)
            print(x)
        except KeyboardInterrupt:
            # スレッドを強制停止
            is_thread_exit = True
            thread.join()
            break


if __name__ == '__main__':
    main()
