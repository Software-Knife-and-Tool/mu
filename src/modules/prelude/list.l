;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; mu lists
;;;
#|
(defun zip (list-1 list-2)
  (errorp-unless listp list-1 "zip: not a list")
  (errorp-unless listp list-2 "zip: not a list")
  (letf ((loop (l1 l2)
          (when l1
            (core::append
             (list (list (car l1) (car l2)))
             (loop (cdr l1) (cdr l2))))))
    (loop list-1 list-2)))
|#
