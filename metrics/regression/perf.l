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
       (lib:difference stop start))
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
                     (:if (lib:less-than offset (lib:vector-len start-vec))
                          ((:lambda (type total alloc in-use)
                             ((:lambda (type-stats)
                                (lib:cons `(,@list ,@type-stats) (lib:sum 1 n)))
                              `(,type ,total ,alloc ,in-use)))
                           (lib:vector-ref start-vec (lib:sum offset 0))
                           (lib:difference (lib:vector-ref stop-vec (lib:sum offset 1)) (lib:vector-ref start-vec (lib:sum offset 1)))
                           (lib:difference (lib:vector-ref stop-vec (lib:sum offset 2)) (lib:vector-ref start-vec (lib:sum offset 2)))
                           (lib:difference (lib:vector-ref stop-vec (lib:sum offset 3)) (lib:vector-ref start-vec (lib:sum offset 3))))
                          (lib:make-vector :t list)))
                         (lib:product n 4)))
                 (lib:car loop)
                 (lib:cdr loop))))
         (lib:cons () 1)))
     (lib:heap-stat)
     ((:lambda ()
         (lib:apply fn args)
         (lib:heap-stat))))))
