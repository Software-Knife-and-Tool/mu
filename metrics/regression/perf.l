;;;  SPDX-FileCopyrightText: Copyright 2017 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; return the elapsed process time in usec
;;; from applying fn to args
;;;
(lib:intern (lib:find-ns "") "perf" (lib:make-ns "perf"))

(lib:intern perf "time-delta"
  (:lambda (fn args)
    ((:lambda (start stop)
       (lib:fx-sub stop start))
       (lib:utime)
       ((:lambda ()
          (lib:apply fn args)
          (lib:utime))))))

;;;
;;; return an vector of storage consumed
;;; by applying fn to args
;;;
(lib:intern perf "storage-delta"
  (:lambda (fn args)
    ((:lambda (start-vec stop-vec)
       (lib:fix
        (:lambda (loop)
          (:if (lib:eq :vector (lib:type-of loop))
               loop
               ((:lambda (list n)
                  ((:lambda (offset)
                     (:if (lib:fx-lt offset (lib:sv-len start-vec))
                          ((:lambda (type total alloc in-use)
                             ((:lambda (type-stats)
                                (lib:cons `(,@list ,@type-stats) (lib:fx-add 1 n)))
                              `(,type ,total ,alloc ,in-use)))
                           (lib:sv-ref start-vec (lib:fx-add offset 0))
                           (lib:fx-sub (lib:sv-ref stop-vec (lib:fx-add offset 1)) (lib:sv-ref start-vec (lib:fx-add offset 1)))
                           (lib:fx-sub (lib:sv-ref stop-vec (lib:fx-add offset 2)) (lib:sv-ref start-vec (lib:fx-add offset 2)))
                           (lib:fx-sub (lib:sv-ref stop-vec (lib:fx-add offset 3)) (lib:sv-ref start-vec (lib:fx-add offset 3))))
                          (lib:vector :t list)))
                         (lib:fx-mul n 4)))
                 (lib:car loop)
                 (lib:cdr loop))))
         (lib:cons () 1)))
     (lib:heap-stat)
     ((:lambda ()
         (lib:apply fn args)
         (lib:heap-stat))))))
